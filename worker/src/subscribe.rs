use crate::run::run_job;
use fallible_iterator::FallibleIterator;
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

// TODO: gracefully handle unwrap failures

pub fn subscribe_to_jobs(database_url: String) {
    let conn = Connection::connect(database_url, TlsMode::None).expect("failed to connect");
    conn.execute("LISTEN job_updates", &[]).unwrap();
    let notifs = conn.notifications();
    loop {
        let _ = notifs.blocking_iter().next();
        conn.execute("BEGIN", &[]).unwrap();
        for row in &conn.query(&*CLAIM_PENDING_JOB_QUERY, &[]).unwrap() {
            let id: i32 = row.get(0);
            let job_type: String = row.get(5);
            let payload: Option<Vec<u8>> = row.get(6);

            let job_result = match JobType::from_str(&job_type) {
                Ok(job_type) => {
                    debug!("running '{}' job", job_type);
                    run_job(id, job_type, payload)
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
