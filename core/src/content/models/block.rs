use chrono::{DateTime, Utc};
use diesel::prelude::*;

// use crate::schema::{blocks, blocks::dsl::blocks as all_blocks};
use crate::content::models::block_version::BlockVersion;
use crate::content::models::note_version::NoteVersion;
use crate::db::{begin_txn, commit_txn};
use crate::errors::ApiError;
use crate::resource_identifier::*;
use crate::schema::blocks;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct Block {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(skip)]
    pub note_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[table_name = "blocks"]
struct BlockCreateSpec {
    pub api_id: String,
    pub note_id: i32,
}
impl BlockCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Block, ApiError> {
        info!("creating block: {:?}", self);
        Ok(diesel::insert_into(blocks::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl Block {
    pub fn create(conn: &PgConnection, note_version: &NoteVersion) -> Result<Block, ApiError> {
        begin_txn(&conn)?;

        let block = BlockCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Block),
            note_id: note_version.note_id,
        }
        .insert(conn)?;
        BlockVersion::create(conn, note_version, &block)?;

        commit_txn(&conn)?;
        Ok(block)
    }
}
