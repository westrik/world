use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
// use crate::schema::{blocks, blocks::dsl::blocks as all_blocks};
use crate::content::models::block_version::BlockVersion;
use crate::db::{begin_txn, commit_txn};
use crate::errors::ApiError;
use crate::resource_identifier::*;
use crate::schema::blocks;
use crate::content::models::note::Note;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Block {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    pub note_id: Option<i32>,
    pub position: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[table_name = "blocks"]
struct BlockCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub note_id: Option<i32>,
    pub position: Option<i32>,
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
    pub fn create(conn: &PgConnection, session: Session, note: Option<&Note>, position: Option<i32>) -> Result<Block, ApiError> {
        begin_txn(&conn)?;

        let block = BlockCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Block),
            user_id: session.user_id,
            note_id: match note {
                Some(n) => Some(n.id),
                _ => None,
            },
            position,
        }
        .insert(conn)?;
        BlockVersion::create(conn, &block)?;

        commit_txn(&conn)?;
        Ok(block)
    }
}
