use chrono::{DateTime, Utc};

use crate::auth::models::user::User;
use crate::schema::{notes, notes::dsl::notes as all_notes};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use diesel::{PgConnection, RunQueryDsl};

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

#[derive(Debug)]
pub enum NoteQueryError {
    NoteNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NoteCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub content: serde_json::Value,
}
impl NoteCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Note, NoteQueryError> {
        info!("{:?}", self);
        Ok(diesel::insert_into(notes::table)
            .values(self)
            .get_result(conn)
            .map_err(NoteQueryError::DatabaseError)?)
    }
}
#[derive(Debug, Deserialize)]
pub struct ApiNoteCreateSpec {
    pub content: serde_json::Value,
}
