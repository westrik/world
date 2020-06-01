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
#[table_name = "note_versions"]
struct NoteVersionCreateSpec {
    pub api_id: String,
    pub note_id: i32,
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
    pub fn create(conn: &PgConnection, note: &Note) -> Result<NoteVersion, ApiError> {
        NoteVersionCreateSpec {
            note_id: note.id,
            api_id: generate_resource_identifier(ResourceType::NoteVersion),
        }
        .insert(conn)
    }
}
