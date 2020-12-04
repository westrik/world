use diesel::prelude::*;

use world_core::auth::models::session::Session;
use world_core::auth::models::user::User;
use world_core::db::{begin_txn, commit_txn, get_conn, DbPool};
use world_core::jobs::errors::JobError;
use world_core::notes::models::note::Note;
use world_core::settings::sites::models::site::Site;
use world_core::settings::sites::models::site_page::{LoadedSitePage, SitePage};

use crate::external_services::s3::put_object_with_custom_credentials;
use crate::sites::render_site::render_site;

fn load_pages_with_content_for_site(
    conn: &PgConnection,
    session: Session,
    site_api_id: String,
) -> Result<Vec<(LoadedSitePage, Note)>, JobError> {
    let pages = {
        let mut pages = SitePage::find_all_for_site(&conn, session.clone(), site_api_id.clone())
            .map_err(JobError::from)?;
        pages.sort_by(|a, b| a.note_version_api_id.cmp(&b.note_version_api_id));
        pages
    };
    let page_content_ids = pages
        .iter()
        .cloned()
        .map(|page| page.note_version_id)
        .collect();
    let page_content = {
        let mut notes = Note::bulk_find(&conn, session, page_content_ids)?;
        notes.sort_by(|a, b| a.version_api_id.cmp(&b.version_api_id));
        notes
    };
    let pages_with_content: Vec<(LoadedSitePage, Note)> = pages
        .iter()
        .cloned()
        .zip(page_content)
        .filter(|(page, note)| {
            note.content.is_some()
                && page
                    .note_version_api_id
                    .eq(&note.version_api_id.clone().unwrap())
        })
        .collect();
    if pages.len() != pages_with_content.len() {
        info!(
            "Filtered out at least one site-page when rendering [site_api_id={}]",
            site_api_id
        );
    }
    Ok(pages_with_content)
}

pub async fn sync_site_to_bucket(
    user_id: i32,
    site_api_id: String,
    pool: &DbPool,
) -> Result<String, JobError> {
    let conn = get_conn(&pool).unwrap();
    begin_txn(&conn).map_err(JobError::from)?;

    let user = User::find_by_id(user_id, &conn).map_err(JobError::from)?;
    let session = Session::create(&conn, &user).map_err(JobError::from)?;
    let site = Site::find_by_api_id(&conn, session.clone(), site_api_id.clone())
        .map_err(JobError::from)?;
    let pages_with_content = load_pages_with_content_for_site(&conn, session, site_api_id.clone())?;

    let rendered_pages = render_site(site.title, pages_with_content).await;
    let bucket_name = site.bucket_name.ok_or_else(|| {
        JobError::InvalidJob("Site does not have S3 bucket configured".to_string())
    })?;
    let access_key_id = site.bucket_access_key_id.ok_or_else(|| {
        JobError::InvalidJob("Site does not have S3 access key ID configured".to_string())
    })?;
    let access_key_secret = site.bucket_access_key_secret.ok_or_else(|| {
        JobError::InvalidJob("Site does not have S3 access key secret configured".to_string())
    })?;

    // TODO: parallel iter
    for rendered_page in rendered_pages {
        let path = format!("{}/index.html", rendered_page.path);
        put_object_with_custom_credentials(
            access_key_id.clone(),
            access_key_secret.clone(),
            bucket_name.clone(),
            &path,
            rendered_page.content.as_bytes().to_vec(),
        )
        .await?;
    }

    commit_txn(&conn).map_err(JobError::from)?;
    Ok("Successfully synced site to S3".to_string())
}
