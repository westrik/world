use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::notes::models::note_version::NoteVersion;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{site_pages, site_pages::dsl::site_pages as all_site_pages};
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
    pub note_version_id: i32,
    pub path: String,
}

pub struct LoadedSitePage {
    pub site_page: SitePage,
    pub site_api_id: String,
    pub note_version_api_id: String,
}

#[derive(Insertable, Debug)]
#[table_name = "site_pages"]
pub struct SitePageCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub site_id: i32,
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
    pub note_version_id: Option<i32>,
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
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: String,
    pub path: String,
}
#[derive(Debug, Deserialize)]
pub struct ApiSitePageCreateSpec {
    #[serde(rename = "siteId")]
    pub site_api_id: String,
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: String,
    pub path: String,
}
#[derive(Debug, Deserialize)]
#[allow(clippy::option_option)]
pub struct ApiSitePageUpdateSpec {
    #[serde(rename = "siteId")]
    pub site_api_id: String,
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: String,
    pub path: Option<String>,
}

impl From<&LoadedSitePage> for ApiSitePage {
    fn from(page: &LoadedSitePage) -> Self {
        ApiSitePage {
            api_id: page.site_page.api_id.clone(),
            created_at: page.site_page.created_at,
            updated_at: page.site_page.updated_at,
            site_api_id: page.site_api_id.clone(),
            note_version_api_id: page.note_version_api_id.clone(),
            path: page.site_page.path.clone(),
        }
    }
}

impl SitePage {
    pub fn find_all_for_site(
        conn: &PgConnection,
        session: Session,
        _site_api_id: String,
    ) -> Result<Vec<LoadedSitePage>, ApiError> {
        // TODO: limit query to only pages for site with site_api_id
        // TODO: load API IDs for note_version & site associated with each site_page
        let items: Vec<SitePage> = all_site_pages
            .filter(site_pages::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        site_api_id: String,
        note_version_api_id: String,
        path: String,
    ) -> Result<LoadedSitePage, ApiError> {
        // TODO: load DB IDs for site_api_id & note_version_api_id
        let (site_id, note_version_id) = (0, 0);
        unimplemented!();

        let new_page = SitePageCreateSpec {
            api_id: generate_resource_identifier(ResourceType::SitePage),
            user_id: session.user_id,
            site_id,
            note_version_id,
            path,
        };
        Ok(LoadedSitePage {
            site_page: new_page.insert(conn)?,
            site_api_id: site_api_id.clone(),
            note_version_api_id: note_version_api_id.clone(),
        })
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        spec: ApiSitePageUpdateSpec,
    ) -> Result<LoadedSitePage, ApiError> {
        // TODO: validate spec.note_version_api_id & spec.site_api_id, and load DB IDs for each
        let (site_id, note_version_id) = (0, 0);
        unimplemented!();

        Ok(LoadedSitePage {
            site_page: SitePageUpdateSpec {
                updated_at: Utc::now(),
                path: spec.path,
                site_id: Some(site_id),
                note_version_id: Some(note_version_id),
            }
            .update(conn, api_id, session.user_id)?,
            site_api_id: spec.site_api_id.clone(),
            note_version_api_id: spec.note_version_api_id.clone(),
        })
    }
}
