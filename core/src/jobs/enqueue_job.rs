use crate::errors::ApiError;
use crate::jobs::job_type::JobType;
use crate::jobs::models::job::Job;
use diesel::PgConnection;

pub fn enqueue_job(
    conn: &PgConnection,
    user_id: Option<i32>,
    job_type: JobType,
    payload: Option<serde_json::Value>,
) -> Result<Job, ApiError> {
    Ok(Job::create(&conn, user_id, job_type, payload)?)
}
