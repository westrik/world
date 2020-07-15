use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::content::models::note::Note;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::note_versions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Note)]
pub struct NoteVersion {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub note_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub content: serde_json::Value,
}

#[derive(Insertable, Debug)]
#[table_name = "note_versions"]
struct NoteVersionCreateSpec {
    pub api_id: String,
    pub note_id: i32,
    pub content: serde_json::Value,
}
impl NoteVersionCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<NoteVersion, ApiError> {
        info!("creating note_version: {:?}", self);
        Ok(diesel::insert_into(note_versions::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl NoteVersion {
    pub fn create(
        conn: &PgConnection,
        note_id: i32,
        content: serde_json::Value,
    ) -> Result<NoteVersion, ApiError> {
        NoteVersionCreateSpec {
            note_id,
            api_id: generate_resource_identifier(ResourceType::NoteVersion),
            content,
        }
        .insert(conn)
    }
}
