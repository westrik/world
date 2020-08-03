use world_core::jobs::{errors::JobError, job_type::JobType};

use crate::jobs::dummy_job::DummyJob;
use crate::jobs::send_email::{EmailTemplate, SendEmailJob};
use crate::jobs::Runnable;

pub fn run_job(id: i32, job_type: JobType, payload: Option<Vec<u8>>) -> Result<String, JobError> {
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
            // TODO: payload -> SendEmailJob
            SendEmailJob {
                template: EmailTemplate::LoginNotification,
                recipients: vec![],
            }
            .run()
        }
    }
}
