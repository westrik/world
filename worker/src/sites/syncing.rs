use world_core::auth::models::session::Session;
use world_core::auth::models::user::User;
use world_core::db::{begin_txn, commit_txn, get_conn, DbPool};
use world_core::jobs::errors::JobError;
// use world_core::notes::export::html::Html;
use world_core::notes::models::note::Note;
use world_core::settings::sites::models::site::Site;
use world_core::settings::sites::models::site_page::SitePage;

pub async fn sync_site_to_bucket(
    user_id: i32,
    site_api_id: String,
    pool: &DbPool,
) -> Result<String, JobError> {
    let conn = get_conn(&pool).unwrap();
    begin_txn(&conn).map_err(JobError::from)?;

    let user = User::find_by_id(user_id, &conn).map_err(JobError::from)?;
    let session = Session::create(&conn, &user).map_err(JobError::from)?;
    let _site = Site::find_by_api_id(&conn, session.clone(), site_api_id.clone())
        .map_err(JobError::from)?;
    let site_pages =
        SitePage::find_all_for_site(&conn, session.clone(), site_api_id).map_err(JobError::from)?;

    let note_version_ids = site_pages
        .into_iter()
        .map(|page| page.note_version_id)
        .collect();

    let _notes = Note::bulk_find(&conn, session, note_version_ids)?;
    // TODO: zip site_pages and notes, assert Some(note.content) for each
    // for (site_page, note) in pages_with_notes {
    //     let html: Html = note.content.unwrap().render();
    // }
    // TODO: populate page templates
    // TODO: copy all files to S3 bucket

    commit_txn(&conn).map_err(JobError::from)?;
    Ok("Successfully synced site to S3".to_string())
}
