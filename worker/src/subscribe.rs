use crate::run::run_job;
use fallible_iterator::FallibleIterator;
#[cfg(feature = "production")]
use postgres::tls::openssl::OpenSsl;
use postgres::{Connection, TlsMode};
use std::str::FromStr;
use world_core::jobs::errors::JobError;
use world_core::jobs::{job_status::JobStatus, job_type::JobType};

lazy_static! {
    static ref CLAIM_PENDING_JOB_QUERY: String = format!(
        r#"
            UPDATE jobs SET status='{}'
            WHERE id = (
              SELECT id
              FROM jobs
              WHERE status='{}'
              ORDER BY id
              FOR UPDATE SKIP LOCKED
              LIMIT 1
            )
            RETURNING *;
        "#,
        JobStatus::Active,
        JobStatus::Pending
    );
    static ref COMPLETE_JOB_QUERY: String = format!(
        r#"
            UPDATE jobs SET status = '{}' WHERE id = $1
        "#,
        JobStatus::Done
    );
    static ref COMPLETE_JOB_WITH_ERROR_QUERY: String = format!(
        r#"
            UPDATE jobs SET status = '{}' WHERE id = $1
        "#,
        JobStatus::Error
    );
}

#[cfg(feature = "production")]
fn get_connection(database_url: String) -> Result<Connection, JobError> {
    // TODO: enable certificate verification
    let ssl = OpenSsl::new()
        .map_err(|_| JobError::InternalError("Failed to load RDS TLS certificate".to_string()))?;
    Ok(
        Connection::connect(database_url, TlsMode::Prefer(&ssl)).map_err(|err| {
            JobError::InternalError(format!("Failed to connect to database: {:#?}", err))
        })?,
    )
}

#[cfg(not(feature = "production"))]
fn get_connection(database_url: String) -> Result<Connection, JobError> {
    Ok(Connection::connect(database_url, TlsMode::None)
        .map_err(|_| JobError::InternalError("Failed to connect to database".to_string()))?)
}

// TODO: gracefully handle unwrap failures

pub async fn subscribe_to_jobs(database_url: String) -> Result<(), JobError> {
    debug!("connecting to database...");

    let conn = get_connection(database_url)?;

    debug!("database connection established");
    conn.execute("LISTEN job_updates", &[])
        .map_err(|err| JobError::DatabaseError(err.to_string()))?;
    let notifs = conn.notifications();
    loop {
        // TODO: don't wait for first notification before claiming pending jobs
        let _ = notifs.blocking_iter().next();
        conn.execute("BEGIN", &[]).unwrap();
        debug!("started txn for job processing");
        for row in &conn.query(&*CLAIM_PENDING_JOB_QUERY, &[]).unwrap() {
            let id: i32 = row.get(0);
            debug!("received job [id={}]", id);
            let job_type: String = row.get(5);
            let payload: Option<serde_json::Value> = row.get(6);

            let job_result = match JobType::from_str(&job_type) {
                Ok(job_type) => {
                    debug!("running '{}' job", job_type);
                    run_job(id, job_type, payload).await
                }
                _ => Err(JobError::InvalidJob(format!(
                    "invalid job type: {}",
                    job_type
                ))),
            };
            match job_result {
                Ok(resp) => {
                    info!("job completed successfully [id={}][response={}]", id, resp);
                    conn.execute(&*COMPLETE_JOB_QUERY, &[&id]).unwrap();
                }
                Err(err) => {
                    error!("job completed with error [id={}][err={:#?}]", id, err);
                    conn.execute(&*COMPLETE_JOB_WITH_ERROR_QUERY, &[&id])
                        .unwrap();
                }
            }
        }
        conn.execute("COMMIT", &[]).unwrap();
    }
}
