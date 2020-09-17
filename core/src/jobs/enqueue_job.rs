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
    let job = Job::create(&conn, user_id, job_type, payload)?;
    let user_id_tag = if let Some(uid) = user_id {
        uid.to_string()
    } else {
        "none".to_string()
    };
    info!(
        "enqueued {} job [user_id={}][job_id={}]",
        job_type, user_id_tag, &job.id
    );
    Ok(job)
}
