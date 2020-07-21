use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::library::models::library_item::LibraryItem;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::library_item_versions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(LibraryItem)]
pub struct LibraryItemVersion {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    pub library_item_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "assetUrl")]
    pub asset_url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "library_item_versions"]
struct LibraryItemVersionCreateSpec {
    pub api_id: String,
    pub library_item_id: i32,
    pub asset_url: String,
}
impl LibraryItemVersionCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<LibraryItemVersion, ApiError> {
        info!("creating library_item_version: {:?}", self);
        Ok(diesel::insert_into(library_item_versions::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl LibraryItemVersion {
    pub fn create(
        conn: &PgConnection,
        library_item_id: i32,
        asset_url: String,
    ) -> Result<LibraryItemVersion, ApiError> {
        LibraryItemVersionCreateSpec {
            library_item_id,
            api_id: generate_resource_identifier(ResourceType::LibraryItemVersion),
            asset_url,
        }
            .insert(conn)
    }
}
