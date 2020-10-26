use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::json;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::jobs::{enqueue_job::enqueue_job, job_type::JobType};
use crate::notes::models::note::Note;
use crate::notes::models::note_version::NoteVersion;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{
    note_versions, notes, site_pages, site_pages::dsl::site_pages as all_site_pages, sites,
};
use crate::settings::sites::models::site::Site;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct SitePage {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub site_id: i32,
    pub note_id: i32,
    pub note_version_id: i32,
    pub path: String,
    pub published: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct LoadedSitePage {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub site_id: i32,
    pub note_id: i32,
    pub note_version_id: i32,
    pub path: String,
    pub published: bool,
    pub site_api_id: String,
    pub note_api_id: String,
    pub note_version_api_id: String,
}

#[derive(Insertable, Debug)]
#[table_name = "site_pages"]
pub struct SitePageCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub site_id: i32,
    pub note_id: i32,
    pub note_version_id: i32,
    pub path: String,
}
impl SitePageCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<SitePage, ApiError> {
        info!("creating site page: {:#?}", self);
        Ok(diesel::insert_into(site_pages::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[allow(clippy::option_option)]
#[derive(AsChangeset, Debug)]
#[table_name = "site_pages"]
pub struct SitePageUpdateSpec {
    // TODO: use trigger to set updated_at automatically
    pub updated_at: DateTime<Utc>,
    pub path: Option<String>,
    pub site_id: Option<i32>,
    pub note_id: Option<i32>,
    pub note_version_id: Option<i32>,
    pub published: Option<bool>,
}
impl SitePageUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<SitePage, ApiError> {
        info!("updating site page {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_site_pages
                .filter(site_pages::api_id.eq(&api_id))
                .filter(site_pages::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<SitePage>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

#[derive(Serialize)]
pub struct ApiSitePage {
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "siteId")]
    pub site_api_id: String,
    #[serde(rename = "noteId")]
    pub note_api_id: String,
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: String,
    pub path: String,
    pub published: bool,
}
#[derive(Debug, Deserialize)]
pub struct ApiSitePageCreateSpec {
    #[serde(rename = "noteId")]
    pub note_api_id: String,
    pub path: String,
}
#[derive(Debug, Deserialize)]
#[allow(clippy::option_option)]
pub struct ApiSitePageUpdateSpec {
    #[serde(rename = "siteId")]
    pub site_api_id: String,
    #[serde(rename = "noteId")]
    pub note_api_id: String,
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: Option<String>,
    pub path: Option<String>,
    pub published: Option<bool>,
}

impl From<&LoadedSitePage> for ApiSitePage {
    fn from(page: &LoadedSitePage) -> Self {
        ApiSitePage {
            api_id: page.api_id.clone(),
            created_at: page.created_at,
            updated_at: page.updated_at,
            path: page.path.clone(),
            site_api_id: page.site_api_id.clone(),
            note_api_id: page.note_api_id.clone(),
            note_version_api_id: page.note_version_api_id.clone(),
            published: page.published,
        }
    }
}

impl SitePage {
    pub fn find_all_for_site(
        conn: &PgConnection,
        session: Session,
        site_api_id: String,
    ) -> Result<Vec<LoadedSitePage>, ApiError> {
        let items: Vec<LoadedSitePage> = all_site_pages
            .filter(site_pages::user_id.eq(session.user_id))
            .inner_join(notes::table)
            .inner_join(note_versions::table)
            .inner_join(sites::table)
            .select((
                site_pages::id,
                site_pages::api_id,
                site_pages::user_id,
                site_pages::created_at,
                site_pages::updated_at,
                site_pages::site_id,
                site_pages::note_id,
                site_pages::note_version_id,
                site_pages::path,
                site_pages::published,
                sites::api_id,
                notes::api_id,
                note_versions::api_id,
            ))
            .filter(sites::api_id.eq(&site_api_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        site_api_id: String,
        note_api_id: String,
        path: String,
    ) -> Result<LoadedSitePage, ApiError> {
        let site = Site::find_by_api_id(conn, session.clone(), site_api_id.clone())?;

        let note = Note::find(conn, session.clone(), note_api_id.clone())?;
        let note_version = {
            if let Some(note_version_api_id) = note.version_api_id {
                Ok(NoteVersion::find_by_api_id(
                    conn,
                    session.clone(),
                    note_version_api_id,
                )?)
            } else {
                Err(ApiError::InvalidRequest(
                    "No versions exist for that note".to_string(),
                ))
            }
        }?;
        let new_page = SitePageCreateSpec {
            api_id: generate_resource_identifier(ResourceType::SitePage),
            user_id: session.user_id,
            site_id: site.id,
            note_id: note.id,
            note_version_id: note_version.id,
            path,
        };
        let page = new_page.insert(conn)?;
        enqueue_job(
            conn,
            Some(page.user_id),
            JobType::SyncSiteToBucket,
            Some(json!({"site_api_id": &site.api_id})),
        )?;
        Ok(LoadedSitePage {
            id: page.id,
            api_id: page.api_id,
            user_id: page.user_id,
            created_at: page.created_at,
            updated_at: page.updated_at,
            site_id: page.site_id,
            note_id: page.note_id,
            note_version_id: page.note_version_id,
            path: page.path,
            published: page.published,
            site_api_id,
            note_api_id,
            note_version_api_id: note_version.api_id,
        })
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        spec: ApiSitePageUpdateSpec,
    ) -> Result<LoadedSitePage, ApiError> {
        let site = Site::find_by_api_id(conn, session.clone(), spec.site_api_id.clone())?;
        let note = Note::find(conn, session.clone(), spec.note_api_id.clone())?;
        let note_version = {
            if let Some(note_version_api_id) = spec.note_version_api_id {
                Ok(NoteVersion::find_by_api_id(
                    conn,
                    session.clone(),
                    note_version_api_id,
                )?)
            } else if let Some(note_version_api_id) = note.version_api_id {
                Ok(NoteVersion::find_by_api_id(
                    conn,
                    session.clone(),
                    note_version_api_id,
                )?)
            } else {
                Err(ApiError::InvalidRequest(
                    "No versions exist for that note".to_string(),
                ))
            }
        }?;
        let page = SitePageUpdateSpec {
            updated_at: Utc::now(),
            path: spec.path,
            site_id: Some(site.id),
            note_id: Some(note.id),
            note_version_id: Some(note_version.id),
            published: if let Some(published) = spec.published {
                Some(published)
            } else {
                None
            },
        }
        .update(conn, api_id, session.user_id)?;
        enqueue_job(
            conn,
            Some(page.user_id),
            JobType::SyncSiteToBucket,
            Some(json!({"site_api_id": &site.api_id})),
        )?;
        Ok(LoadedSitePage {
            id: page.id,
            api_id: page.api_id,
            user_id: page.user_id,
            created_at: page.created_at,
            updated_at: page.updated_at,
            site_id: page.site_id,
            note_id: page.note_id,
            note_version_id: page.note_version_id,
            path: page.path,
            published: page.published,
            site_api_id: spec.site_api_id,
            note_api_id: note.api_id,
            note_version_api_id: note_version.api_id,
        })
    }
}
