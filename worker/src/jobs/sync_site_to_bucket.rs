use async_trait::async_trait;

use world_core::db::DbPool;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;
use crate::sites::syncing::sync_site_to_bucket;

#[derive(Serialize, Deserialize)]
pub struct SyncSiteToBucketJob {
    pub site_api_id: String,
}

#[async_trait]
impl Runnable for SyncSiteToBucketJob {
    async fn run(&self, db_pool: &DbPool, user_id: Option<i32>) -> Result<String, JobError> {
        info!(
            "syncing site to S3 bucket [site_api_id={}][user_id={:?}]",
            self.site_api_id, user_id
        );
        if let Some(user_id) = user_id {
            sync_site_to_bucket(user_id, self.site_api_id.clone(), db_pool).await
        } else {
            Err(JobError::InvalidJob(
                "SyncSiteToBucketJob was not associated with a user".to_string(),
            ))
        }
    }
}
