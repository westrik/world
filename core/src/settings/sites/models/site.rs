use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{sites, sites::dsl::sites as all_sites};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Site {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub description: String,
}

#[derive(Insertable, Debug)]
#[table_name = "sites"]
pub struct SiteCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub description: String,
}
impl SiteCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Site, ApiError> {
        info!("creating site: {:#?}", self);
        Ok(diesel::insert_into(sites::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[allow(clippy::option_option)]
#[derive(AsChangeset, Debug)]
#[table_name = "sites"]
pub struct SiteUpdateSpec {
    // TODO: use trigger to set updated_at automatically
    pub updated_at: DateTime<Utc>,
    pub description: Option<String>,
}
impl SiteUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<Site, ApiError> {
        info!("updating site {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_sites
                .filter(sites::api_id.eq(&api_id))
                .filter(sites::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<Site>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

#[derive(Serialize)]
pub struct ApiSite {
    #[serde(rename = "id")]
    pub api_id: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Deserialize)]
pub struct ApiSiteCreateSpec {
    pub description: String,
}
#[derive(Debug, Deserialize)]
#[allow(clippy::option_option)]
pub struct ApiSiteUpdateSpec {
    pub description: Option<String>,
}

impl From<&Site> for ApiSite {
    fn from(site: &Site) -> Self {
        ApiSite {
            api_id: site.api_id.clone(),
            description: site.description.clone(),
            created_at: site.created_at,
            updated_at: site.updated_at,
        }
    }
}

impl Site {
    pub fn find_all_for_user(conn: &PgConnection, session: Session) -> Result<Vec<Site>, ApiError> {
        let items: Vec<Site> = all_sites
            .filter(sites::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(items)
    }

    pub fn find_by_api_id(
        conn: &PgConnection,
        session: Session,
        api_id: String,
    ) -> Result<Site, ApiError> {
        let item: Site = all_sites
            .filter(sites::user_id.eq(session.user_id))
            .filter(sites::api_id.eq(api_id))
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(item)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        description: String,
    ) -> Result<Site, ApiError> {
        let new_site = SiteCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Site),
            user_id: session.user_id,
            description,
        };
        new_site.insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        spec: ApiSiteUpdateSpec,
    ) -> Result<Site, ApiError> {
        SiteUpdateSpec {
            updated_at: Utc::now(),
            description: spec.description,
        }
        .update(conn, api_id, session.user_id)
    }
}
