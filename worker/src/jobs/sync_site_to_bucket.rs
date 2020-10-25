use async_trait::async_trait;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;

#[derive(Serialize, Deserialize)]
pub struct SyncSiteToBucketJob {
    pub site_api_id: String,
}

#[async_trait]
impl Runnable for SyncSiteToBucketJob {
    async fn run(&self) -> Result<String, JobError> {
        // TODO: validate input
        info!(
            "syncing site to S3 bucket [site_api_id={}]",
            self.site_api_id
        );
        // TODO: export all site pages to HTML & populate page templates
        // TODO: copy all files to S3 bucket
        Ok("DONE".to_string())
    }
}
