use fallible_iterator::FallibleIterator;
use postgres::{Connection, TlsMode};

pub fn subscribe_to_jobs(database_url: String) {
    let conn = Connection::connect(database_url, TlsMode::None).expect("Failed to connect");
    conn.execute("LISTEN job_updates", &[]).unwrap();
    let notifs = conn.notifications();
    loop {
        let _ = notifs.blocking_iter().next();
        conn.execute("BEGIN", &[]).unwrap();
        for row in &conn
            .query(
                r#"
            UPDATE jobs SET status='running'
            WHERE id = (
              SELECT id
              FROM jobs
              WHERE status='new'
              ORDER BY id
              FOR UPDATE SKIP LOCKED
              LIMIT 1
            )
            RETURNING *;
        "#,
                &[],
            )
            .unwrap()
        {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let status: String = row.get(2);
            println!(
                "processing job with id: {:?}, name: {:?}, status: {:?}",
                id, name, status
            );
            conn.execute("UPDATE jobs SET status = 'success' WHERE id = $1", &[&id])
                .unwrap();
        }
        conn.execute("COMMIT", &[]).unwrap();
    }
}
