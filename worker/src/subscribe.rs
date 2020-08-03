use fallible_iterator::FallibleIterator;
use postgres::{Connection, TlsMode};
use world_core::jobs::job_status::JobStatus;

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
}

pub fn subscribe_to_jobs(database_url: String) {
    let conn = Connection::connect(database_url, TlsMode::None).expect("Failed to connect");
    conn.execute("LISTEN job_updates", &[]).unwrap();
    let notifs = conn.notifications();
    loop {
        let _ = notifs.blocking_iter().next();
        conn.execute("BEGIN", &[]).unwrap();
        for row in &conn.query(&*CLAIM_PENDING_JOB_QUERY, &[]).unwrap() {
            let id: i32 = row.get(0);
            let api_id: String = row.get(1);
            // let created_at: DateTime<Utc> = row.get(2);
            // let updated_at: DateTime<Utc> = row.get(3);
            let status: String = row.get(4);
            let job_type: String = row.get(5);
            let payload: Vec<u8> = row.get(6);
            println!(
                "processing job with api_id: {:?}, type: {:?}, status: {:?}, payload: {:?}",
                api_id, job_type, status, payload
            );
            // TODO: add task to tokio queue
            conn.execute(&*COMPLETE_JOB_QUERY, &[&id]).unwrap();
        }
        conn.execute("COMMIT", &[]).unwrap();
    }
}
