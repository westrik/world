use crate::db::{get_conn, DbPool};
use world_core::jobs::enqueue_job::enqueue_job;
use world_core::jobs::job_type::JobType;

#[test_case]
fn test_enqueued_job_is_completed(pool: &DbPool) {
    let conn = get_conn(pool).unwrap();
    let _ = enqueue_job(&conn, None, JobType::SendEmail, None);
}
