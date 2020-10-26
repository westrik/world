use async_trait::async_trait;
use diesel::Connection;

use world_core::auth::models::user::User;
use world_core::db;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;
use world_core::auth::models::session::Session;
use world_core::settings::sites::models::site::Site;
use world_core::settings::sites::models::site_page::SitePage;

#[derive(Serialize, Deserialize)]
pub struct SyncSiteToBucketJob {
    pub site_api_id: String,
}

async fn sync_site_to_bucket(
    user_id: i32,
    site_api_id: String,
    pool: &db::DbPool,
) -> Result<String, JobError> {
    let conn = db::get_conn(&pool).unwrap();
    conn.execute("BEGIN")
        .map_err(|e| JobError::DatabaseError(e.to_string()))?;

    let user = User::find_by_id(user_id, &conn).map_err(JobError::from)?;
    let session = Session::create(&conn, &user).map_err(JobError::from)?;
    let _site = Site::find_by_api_id(&conn, session.clone(), site_api_id.clone())
        .map_err(JobError::from)?;
    let _site_pages =
        SitePage::find_all_for_site(&conn, session, site_api_id).map_err(JobError::from)?;
    // TODO: export all pages to HTML & populate page templates
    // TODO: copy all files to S3 bucket
    conn.execute("COMMIT")
        .map_err(|e| JobError::DatabaseError(e.to_string()))?;
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
