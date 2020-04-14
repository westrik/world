use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::json;

use crate::content::models::block::Block;
// use crate::schema::{block_versions, block_versions::dsl::block_versions as all_block_versions};
use crate::errors::ApiError;
use crate::schema::block_versions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Block)]
pub struct BlockVersion {
    #[serde(skip)]
    pub id: i32,
    #[serde(skip)]
    pub block_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub content: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[table_name = "block_versions"]
struct BlockVersionCreateSpec {
    pub block_id: i32,
    pub content: serde_json::Value,
}
impl BlockVersionCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<BlockVersion, ApiError> {
        info!("creating block_version: {:?}", self);
        Ok(diesel::insert_into(block_versions::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl BlockVersion {
    pub fn create(conn: &PgConnection, block: &Block) -> Result<BlockVersion, ApiError> {
        BlockVersionCreateSpec {
            block_id: block.id,
            content: json!({"elements": []}),
        }
        .insert(conn)
    }
}
