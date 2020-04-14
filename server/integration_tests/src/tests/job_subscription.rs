use crate::db::{get_conn, DbPool};
use westrikworld_core::jobs::job_type::JobType;
use westrikworld_core::jobs::models::job::Job;

#[test_case]
fn test_enqueued_job_is_completed(pool: &DbPool) {
    let conn = get_conn(pool).unwrap();
    let _tasks = Job::create(
        &conn,
        JobType::SYSTEM,
        Some(serde_json::from_str("{}").unwrap()),
        None,
    );
}
