use async_trait::async_trait;
use world_core::jobs::errors::JobError;

pub mod ingest_media_upload;
pub mod send_email;
pub mod sync_site_to_bucket;

#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<String, JobError>;
}
