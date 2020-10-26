use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::media::models::media_item::MediaItemSummary;
use crate::media::models::media_item_version_type::MediaItemVersionType;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::media_item_versions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(MediaItemSummary, foreign_key = "media_item_id")]
#[belongs_to(User)]
pub struct MediaItemVersion {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    pub media_item_id: i32,
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
#[table_name = "media_item_versions"]
struct MediaItemVersionCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub media_item_id: i32,
    pub version_type: String,
    pub asset_url: Option<String>,
    pub asset_file_size_bytes: Option<i64>,
}
impl MediaItemVersionCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<MediaItemVersion, ApiError> {
        info!("creating media_item_version: {:?}", self);
        Ok(diesel::insert_into(media_item_versions::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl MediaItemVersion {
    pub fn create(
        conn: &PgConnection,
        user_id: i32,
        media_item_summary: MediaItemSummary,
        version_type: MediaItemVersionType,
        asset_url: Option<String>,
    ) -> Result<MediaItemVersion, ApiError> {
        MediaItemVersionCreateSpec {
            api_id: generate_resource_identifier(ResourceType::MediaItemVersion),
            media_item_id: media_item_summary.id,
            user_id,
            asset_url,
            version_type: version_type.to_string(),
            asset_file_size_bytes: media_item_summary.uploaded_file_size_bytes,
        }
        .insert(conn)
    }
}
