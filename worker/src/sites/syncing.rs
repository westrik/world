use world_core::auth::models::session::Session;
use world_core::auth::models::user::User;
use world_core::db::{begin_txn, commit_txn, get_conn, DbPool};
use world_core::jobs::errors::JobError;
use world_core::notes::export::{html::Html, Render};
use world_core::notes::models::note::Note;
use world_core::notes::schema::Content;
use world_core::settings::sites::models::site::Site;
use world_core::settings::sites::models::site_page::{LoadedSitePage, SitePage};

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

    let site_pages = {
        let mut site_pages =
            SitePage::find_all_for_site(&conn, session.clone(), site_api_id.clone())
                .map_err(JobError::from)?;
        site_pages.sort_by(|a, b| a.note_version_api_id.cmp(&b.note_version_api_id));
        site_pages
    };
    let note_version_ids = site_pages
        .iter()
        .cloned()
        .map(|page| page.note_version_id)
        .collect();

    let notes = {
        let mut notes = Note::bulk_find(&conn, session, note_version_ids)?;
        notes.sort_by(|a, b| a.version_api_id.cmp(&b.version_api_id));
        notes
    };
    let pages_with_notes: Vec<(LoadedSitePage, Note)> = site_pages
        .iter()
        .cloned()
        .zip(notes)
        .filter(|(page, note)| {
            (note.content.is_some()
                && page
                    .note_version_api_id
                    .eq(&note.version_api_id.clone().unwrap()))
        })
        .collect();

    if site_pages.len() != pages_with_notes.len() {
        info!(
            "Filtered out at least one site-page when rendering [site_api_id={}]",
            site_api_id
        );
    }

    for (_site_page, note) in pages_with_notes {
        let content: Content = serde_json::from_value(note.content.unwrap()).unwrap();
        let _html: Html = content.render();
    }
    // TODO: populate page templates
    // TODO: copy all files to S3 bucket

    commit_txn(&conn).map_err(JobError::from)?;
    Ok("Successfully synced site to S3".to_string())
}
