use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::auth::models::session::Session;
use crate::errors::ApiError;
use crate::notes::models::note::NoteSummary as Note;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{note_versions, note_versions::dsl::note_versions as all_note_versions, notes};

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
    pub fn find_by_api_id(
        conn: &PgConnection,
        session: Session,
        api_id: String,
    ) -> Result<NoteVersion, ApiError> {
        Ok(all_note_versions
            .inner_join(notes::table)
            .select(note_versions::all_columns)
            .filter(notes::user_id.eq(session.user_id))
            .filter(note_versions::api_id.eq(api_id))
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }

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
