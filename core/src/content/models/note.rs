use chrono::{DateTime, Utc};

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::content::models::note_version::NoteVersion;
use crate::db::{begin_txn, commit_txn, rollback_txn};
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{notes, notes::dsl::notes as all_notes};
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};
use diesel::prelude::*;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Note {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct NoteSummary {
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NoteCreateSpec {
    pub api_id: String,
    pub name: String,
    pub user_id: i32,
}
impl NoteCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Note, ApiError> {
        info!("creating note: {:?}", self);
        Ok(diesel::insert_into(notes::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[derive(AsChangeset, Debug)]
#[table_name = "notes"]
pub struct NoteUpdateSpec {
    pub updated_at: DateTime<Utc>, // TODO: use trigger to set updated_at automatically
    pub name: Option<String>,
}
impl NoteUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<Note, ApiError> {
        info!("updating note {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_notes
                .filter(notes::api_id.eq(&api_id))
                .filter(notes::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<Note>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

impl Note {
    pub fn find_all(conn: &PgConnection, session: Session) -> Result<Vec<NoteSummary>, ApiError> {
        let notes: Vec<NoteSummary> = all_notes
            .select((notes::api_id, notes::created_at, notes::updated_at))
            .filter(notes::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(notes)
    }

    pub fn find(conn: &PgConnection, session: Session, api_id: String) -> Result<Note, ApiError> {
        let note = all_notes
            .filter(notes::user_id.eq(session.user_id))
            .filter(notes::api_id.eq(&api_id))
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(api_id),
                _ => ApiError::DatabaseError(e),
            })?;
        Ok(note)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        content: Option<serde_json::Value>,
    ) -> Result<Note, ApiError> {
        begin_txn(conn).unwrap();

        let note_result = NoteCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Note),
            name: generate_mnemonic(DEFAULT_MNEMONIC_LENGTH),
            user_id: session.user_id,
        }
        .insert(conn);

        if let Ok(created_note) = note_result {
            if let Some(content_data) = content {
                let note_version = NoteVersion::create(conn, created_note.id, content_data);
                if let Ok(_created_note_version) = note_version {
                    // TODO: log note version creation
                    commit_txn(conn).unwrap();
                    Ok(created_note)
                } else {
                    // TODO: handle failure
                    rollback_txn(conn).unwrap();
                    Err(ApiError::InternalError(
                        "Failed to create note version".to_string(),
                    ))
                }
            } else {
                Ok(created_note)
            }
        } else {
            // TODO: handle failure
            rollback_txn(conn).unwrap();
            Err(ApiError::InternalError("Failed to create note".to_string()))
        }
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        name: Option<String>,
    ) -> Result<Note, ApiError> {
        NoteUpdateSpec {
            updated_at: Utc::now(),
            name,
        }
        .update(conn, api_id, session.user_id)
    }
}

#[cfg(test)]
pub mod note_queries {
    use super::*;
    use crate::schema::{notes, notes::dsl::notes as all_notes};
    use diesel::pg::Pg;

    #[test]
    fn get_note() {
        let query = all_notes
            .select((notes::api_id, notes::created_at, notes::updated_at))
            .filter(notes::user_id.eq(13));
        let debug_query = diesel::debug_query::<Pg, _>(&query);
        println!("{}", debug_query);
    }
}
