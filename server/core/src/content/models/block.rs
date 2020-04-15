use chrono::{DateTime, Utc};
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
// use crate::schema::{blocks, blocks::dsl::blocks as all_blocks};
use crate::schema::blocks;

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

impl Block {
    pub fn create(conn: &PgConnection, session: Session) -> Block {}
}
