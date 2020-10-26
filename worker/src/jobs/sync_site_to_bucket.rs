use async_trait::async_trait;

use world_core::auth::models::user::User;
use world_core::db;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;

#[derive(Serialize, Deserialize)]
pub struct SyncSiteToBucketJob {
    pub site_api_id: String,
}

async fn sync_site_to_bucket(
    user_id: i32,
    _site_api_id: String,
    pool: &db::DbPool,
) -> Result<String, JobError> {
    let conn = db::get_conn(&pool).unwrap();
    let _user = User::find_by_id(user_id, &conn)
        .map_err(|e| JobError::InternalError(format!("Failed to load user [err={:?}", e)))?;
    // TODO: load site
    // TODO: load all pages for site
    // TODO: export all pages to HTML & populate page templates
    // TODO: copy all files to S3 bucket
    Ok("Successfully synced site to S3".to_string())
}

#[async_trait]
impl Runnable for SyncSiteToBucketJob {
    async fn run(&self, db_pool: &db::DbPool, user_id: Option<i32>) -> Result<String, JobError> {
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
