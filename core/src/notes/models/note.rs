use chrono::{DateTime, Utc};

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::db::{begin_txn, commit_txn, rollback_txn};
use crate::errors::ApiError;
use crate::notes::models::note_version::NoteVersion;
use crate::notes::parsing::parse_markdown_content;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::note_versions;
use crate::schema::{notes, notes::dsl::notes as all_notes};
use crate::utils::list_options::ListOptions;
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};
use diesel::prelude::*;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[table_name = "notes"]
pub struct NoteSummary {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Note {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(rename = "versionId")]
    pub version_api_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub content: Option<serde_json::Value>,
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NoteCreateSpec {
    pub api_id: String,
    pub name: String,
    pub user_id: i32,
}
impl NoteCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<NoteSummary, ApiError> {
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
    // TODO: use trigger to set updated_at automatically
    pub updated_at: DateTime<Utc>,
    pub name: Option<String>,
}
impl NoteUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<NoteSummary, ApiError> {
        info!("updating note {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_notes
                .filter(notes::api_id.eq(&api_id))
                .filter(notes::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<NoteSummary>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

fn create_version_for_note_and_commit(
    conn: &PgConnection,
    note: Result<NoteSummary, ApiError>,
    content: Option<serde_json::Value>,
) -> Result<Note, ApiError> {
    if let Ok(note_) = note {
        if let Some(content_data) = content {
            let note_version = NoteVersion::create(conn, note_.id, content_data);
            if let Ok(created_note_version) = note_version {
                // TODO: log note version creation
                // TODO: move transaction handling out of this fn
                commit_txn(conn).unwrap();
                Ok(Note {
                    id: note_.id,
                    api_id: note_.api_id,
                    created_at: note_.created_at,
                    updated_at: note_.updated_at,
                    name: note_.name,
                    version_api_id: Some(created_note_version.api_id),
                    content: Some(created_note_version.content),
                })
            } else {
                // TODO: handle failure
                // TODO: move transaction handling out of this fn
                rollback_txn(conn).unwrap();
                Err(ApiError::InternalError(
                    "Failed to create note version".to_string(),
                ))
            }
        } else {
            let default_content =
                serde_json::to_value(parse_markdown_content(format!("# {}", note_.name))).unwrap();
            let note_version = NoteVersion::create(conn, note_.id, default_content);
            if let Ok(created_note_version) = note_version {
                // TODO: log note version creation
                // TODO: move transaction handling out of this fn
                commit_txn(conn).unwrap();
                Ok(Note {
                    id: note_.id,
                    api_id: note_.api_id,
                    created_at: note_.created_at,
                    updated_at: note_.updated_at,
                    name: note_.name,
                    version_api_id: Some(created_note_version.api_id),
                    content: None,
                })
            } else {
                // TODO: handle failure
                // TODO: move transaction handling out of this fn
                rollback_txn(conn).unwrap();
                Err(ApiError::InternalError(
                    "Failed to create note version".to_string(),
                ))
            }
        }
    } else {
        // TODO: move transaction handling out of this fn
        rollback_txn(conn).unwrap();
        Err(ApiError::InternalError("Failed to create note".to_string()))
    }
}

type LoadedNote = (
    i32,
    String,
    String,
    DateTime<Utc>,
    DateTime<Utc>,
    String,
    serde_json::Value,
);

impl Note {
    pub fn list(
        conn: &PgConnection,
        session: Session,
        _options: ListOptions,
    ) -> Result<Vec<NoteSummary>, ApiError> {
        let notes: Vec<NoteSummary> = all_notes
            .filter(notes::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(notes)
    }

    pub fn find(conn: &PgConnection, session: Session, api_id: String) -> Result<Note, ApiError> {
        let result = note_versions::table
            .inner_join(notes::table)
            .select((
                notes::id,
                notes::api_id,
                note_versions::api_id,
                notes::created_at,
                notes::updated_at,
                notes::name,
                note_versions::content,
            ))
            .filter(notes::user_id.eq(session.user_id))
            .filter(notes::api_id.eq(&api_id))
            .order(note_versions::id.desc())
            .limit(1)
            .first::<LoadedNote>(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(api_id.to_string()),
                _ => ApiError::DatabaseError(e),
            })?;
        Ok(Note {
            id: result.0,
            api_id: result.1,
            version_api_id: Some(result.2),
            created_at: result.3,
            updated_at: result.4,
            name: result.5,
            content: Some(result.6),
        })
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        name: Option<String>,
        content: Option<serde_json::Value>,
    ) -> Result<Note, ApiError> {
        // TODO: move transaction handling out of this fn
        begin_txn(conn).unwrap();
        let note_result = NoteCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Note),
            name: if let Some(name) = name {
                name
            } else {
                generate_mnemonic(DEFAULT_MNEMONIC_LENGTH)
            },
            user_id: session.user_id,
        }
        .insert(conn);
        create_version_for_note_and_commit(conn, note_result, content)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        name: Option<String>,
        updated_content: Option<serde_json::Value>,
    ) -> Result<Note, ApiError> {
        // TODO: move transaction handling out of this fn
        begin_txn(conn).unwrap();
        let note_result = NoteUpdateSpec {
            updated_at: Utc::now(),
            name,
        }
        .update(conn, api_id, session.user_id);
        create_version_for_note_and_commit(conn, note_result, updated_content)
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
