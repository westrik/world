use chrono::{DateTime, Utc};
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::content::models::block::Block;
// use crate::schema::{block_versions, block_versions::dsl::block_versions as all_block_versions};
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

impl BlockVersion {
    pub fn create(conn: &PgConnection, session: Session, block: Block) -> BlockVersion {}
}
