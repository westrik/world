use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::schema::{
    media_item_versions, media_items, media_items::dsl::media_items as all_media_items,
};
use crate::utils::list_options::ListOptions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[table_name = "media_items"]
pub struct MediaItemSummary {
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
    // TODO: add item_type column
    // TODO: split next three columns out of this model
    #[serde(rename = "preSignedUploadUrl")]
    pub presigned_upload_url: Option<String>,
    #[serde(rename = "uploadedFileName")]
    pub uploaded_file_name: Option<String>,
    #[serde(rename = "uploadedFileSizeBytes")]
    pub uploaded_file_size_bytes: Option<i64>,
}

#[derive(Queryable, Serialize, Debug)]
pub struct MediaItem {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(rename = "versionId")]
    pub version_api_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "assetUrl")]
    pub asset_url: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "media_items"]
pub struct MediaItemCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub name: String,
    pub presigned_upload_url: Option<String>,
    pub uploaded_file_name: Option<String>,
    pub uploaded_file_size_bytes: Option<i64>,
}

#[derive(AsChangeset, Debug)]
#[table_name = "media_items"]
pub struct MediaItemUpdateSpec {
    // TODO: use trigger to set updated_at automatically
    pub updated_at: DateTime<Utc>,
    pub name: Option<String>,
}
impl MediaItemUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<MediaItemSummary, ApiError> {
        info!("updating media_item {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_media_items
                .filter(media_items::api_id.eq(&api_id))
                .filter(media_items::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<MediaItemSummary>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

impl MediaItemSummary {
    pub fn find(
        conn: &PgConnection,
        user_id: i32,
        api_id: String,
    ) -> Result<MediaItemSummary, ApiError> {
        Ok(all_media_items
            .filter(media_items::api_id.eq(&api_id))
            .filter(media_items::user_id.eq(user_id))
            .first::<MediaItemSummary>(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

type DbMediaItem = (
    i32,
    String,
    String,
    DateTime<Utc>,
    DateTime<Utc>,
    String,
    Option<String>,
);

impl MediaItem {
    pub fn list(
        conn: &PgConnection,
        user_id: i32,
        _options: ListOptions,
    ) -> Result<Vec<MediaItem>, ApiError> {
        // TODO: refactor this
        let media_items: Vec<DbMediaItem> = media_item_versions::table
            .inner_join(media_items::table)
            .distinct_on(media_items::id)
            .select((
                media_items::id,
                media_items::api_id,
                media_item_versions::api_id,
                media_items::created_at,
                media_items::updated_at,
                media_items::name,
                media_item_versions::asset_url,
            ))
            .filter(media_items::user_id.eq(user_id))
            .order((media_items::id.desc(), media_item_versions::id.desc()))
            .load::<DbMediaItem>(conn)
            .map_err(|e| {
                println!("{:#?}", e);
                ApiError::DatabaseError(e)
            })?;
        Ok(media_items
            .into_iter()
            .map(|item| MediaItem {
                id: item.0,
                api_id: item.1,
                version_api_id: Some(item.2),
                created_at: item.3,
                updated_at: item.4,
                name: item.5,
                asset_url: item.6,
            })
            .collect())
    }

    pub fn find(conn: &PgConnection, user_id: i32, api_id: String) -> Result<MediaItem, ApiError> {
        // TODO: refactor this
        let item: DbMediaItem = media_item_versions::table
            .inner_join(media_items::table)
            .select((
                media_items::id,
                media_items::api_id,
                media_item_versions::api_id,
                media_items::created_at,
                media_items::updated_at,
                media_items::name,
                media_item_versions::asset_url,
            ))
            .filter(media_items::user_id.eq(user_id))
            .filter(media_items::api_id.eq(api_id))
            .order(media_item_versions::id.desc())
            .limit(1)
            .first::<DbMediaItem>(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(MediaItem {
            id: item.0,
            api_id: item.1,
            version_api_id: Some(item.2),
            created_at: item.3,
            updated_at: item.4,
            name: item.5,
            asset_url: item.6,
        })
    }

    pub fn bulk_create(
        conn: &PgConnection,
        specs: Vec<MediaItemCreateSpec>,
    ) -> Result<Vec<MediaItemSummary>, ApiError> {
        Ok(insert_into(all_media_items)
            .values(specs)
            .get_results(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn update(
        conn: &PgConnection,
        user_id: i32,
        api_id: String,
        name: Option<String>,
    ) -> Result<MediaItemSummary, ApiError> {
        // TODO: handle generating new upload URL (if needed)
        MediaItemUpdateSpec {
            updated_at: Utc::now(),
            name,
        }
        .update(conn, api_id, user_id)
    }
}
