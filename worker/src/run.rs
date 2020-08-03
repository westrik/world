use world_core::jobs::{errors::JobError, job_type::JobType};

use crate::jobs::dummy_job::DummyJob;
use crate::jobs::send_email::SendEmailJob;
use crate::jobs::Runnable;

pub fn run_job(
    id: i32,
    job_type: JobType,
    payload: Option<serde_json::Value>,
) -> Result<String, JobError> {
    info!(
        "processing job [id={:?}][type={:?}][has_payload={:?}]",
        id,
        job_type,
        payload.is_some()
    );

    // TODO: run task on tokio threadpool
    match job_type {
        JobType::DummyJob => DummyJob {}.run(),
        JobType::SendEmail => {
            if let Some(job_data) = payload {
                let job_data: SendEmailJob = serde_json::from_value(job_data).unwrap();
                job_data.run()
            } else {
                Err(JobError::InvalidJob(
                    "No payload provided for send-email job".to_string(),
                ))
            }
        }
    }
}
