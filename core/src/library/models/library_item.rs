use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{library_items, library_items::dsl::library_items as all_library_items};
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct LibraryItem {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "uploadUrl")]
    pub upload_url: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "library_items"]
struct LibraryItemCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub name: String,
}
impl LibraryItemCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<LibraryItem, ApiError> {
        info!("creating library_item: {:?}", self);
        Ok(diesel::insert_into(library_items::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[derive(AsChangeset, Debug)]
#[table_name = "library_items"]
pub struct LibraryItemUpdateSpec {
    pub updated_at: DateTime<Utc>, // TODO: use trigger to set updated_at automatically
    pub name: Option<String>,
}
impl LibraryItemUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<LibraryItem, ApiError> {
        info!("updating library_item {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_library_items
                .filter(library_items::api_id.eq(&api_id))
                .filter(library_items::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<LibraryItem>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

impl LibraryItem {
    pub fn find_all(conn: &PgConnection, session: Session) -> Result<Vec<LibraryItem>, ApiError> {
        Ok(all_library_items
            .filter(library_items::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn find(
        conn: &PgConnection,
        session: Session,
        api_id: String,
    ) -> Result<LibraryItem, ApiError> {
        Ok(all_library_items
            .filter(library_items::user_id.eq(session.user_id))
            .filter(library_items::api_id.eq(api_id))
            .first::<LibraryItem>(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        name: Option<String>,
    ) -> Result<LibraryItem, ApiError> {
        LibraryItemCreateSpec {
            api_id: generate_resource_identifier(ResourceType::LibraryItem),
            user_id: session.user_id,
            name: name.unwrap_or_else(|| generate_mnemonic(DEFAULT_MNEMONIC_LENGTH)),
        }
        .insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        name: Option<String>,
    ) -> Result<LibraryItem, ApiError> {
        LibraryItemUpdateSpec {
            updated_at: Utc::now(),
            name,
        }
        .update(conn, api_id, session.user_id)
    }
}
