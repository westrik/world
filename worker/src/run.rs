use world_core::db::DbPool;
use world_core::jobs::{errors::JobError, job_type::JobType};

use crate::jobs::ingest_media_upload::IngestMediaUploadJob;
use crate::jobs::send_email::SendEmailJob;
use crate::jobs::sync_site_to_bucket::SyncSiteToBucketJob;
use crate::jobs::Runnable;

pub async fn run_job(
    id: i32,
    job_type: JobType,
    user_id: Option<i32>,
    payload: Option<serde_json::Value>,
    db_pool: &DbPool,
) -> Result<String, JobError> {
    info!(
        "processing job [id={:?}][type={:?}][has_payload={:?}]",
        id,
        job_type,
        payload.is_some()
    );

    // TODO: refactor & run task on tokio threadpool (?)
    // TODO: auto-rollback failed tasks on error
    match job_type {
        JobType::IngestMediaUpload => {
            let payload = payload.unwrap();
            let ingest_job: IngestMediaUploadJob = serde_json::from_value(payload).unwrap();
            ingest_job.run(db_pool, user_id).await
        }
        JobType::SendEmail => {
            let payload = payload.unwrap();
            let email_job: SendEmailJob = serde_json::from_value(payload).unwrap();
            email_job.run(db_pool, user_id).await
        }
        JobType::SyncSiteToBucket => {
            let payload = payload.unwrap();
            let sync_job: SyncSiteToBucketJob = serde_json::from_value(payload).unwrap();
            sync_job.run(db_pool, user_id).await
        }
    }
}
