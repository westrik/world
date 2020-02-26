use chrono::{DateTime, Utc};

use crate::auth::models::user::User;
use crate::schema::{notes, notes::dsl::notes as all_notes};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Note {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NoteCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub content: serde_json::Value,
}
