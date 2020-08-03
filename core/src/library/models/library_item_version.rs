use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::library::models::library_item::LibraryItem;
use crate::library::models::library_item_version_type::LibraryItemVersionType;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::library_item_versions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(LibraryItem)]
#[belongs_to(User)]
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
    #[serde(rename = "versionType")]
    pub version_type: String,
    #[serde(rename = "assetUrl")]
    pub asset_url: Option<String>,
    #[serde(rename = "assetFileSizeBytes")]
    pub asset_file_size_bytes: Option<i64>,
    #[serde(skip)] // TODO: don't skip
    pub asset_data: Option<serde_json::Value>,
}

#[derive(Insertable, Debug)]
#[table_name = "library_item_versions"]
struct LibraryItemVersionCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub library_item_id: i32,
    pub version_type: String,
    pub asset_url: Option<String>,
    pub asset_file_size_bytes: Option<i64>,
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
        user_id: i32,
        library_item: LibraryItem,
        version_type: LibraryItemVersionType,
        asset_url: Option<String>,
    ) -> Result<LibraryItemVersion, ApiError> {
        LibraryItemVersionCreateSpec {
            api_id: generate_resource_identifier(ResourceType::LibraryItemVersion),
            library_item_id: library_item.id,
            user_id,
            asset_url,
            version_type: version_type.to_string(),
            asset_file_size_bytes: library_item.uploaded_file_size_bytes,
        }
        .insert(conn)
    }
}
