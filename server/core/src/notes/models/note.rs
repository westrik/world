use chrono::{DateTime, Utc};

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::notes::handlers::ApiNoteUpdateSpec;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{notes, notes::dsl::notes as all_notes};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use diesel::dsl::now;
use diesel::prelude::*;

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
pub enum NoteError {
    // our bad (500s)
    DatabaseError(diesel::result::Error),
    BadContentConversion,

    // their bad (400s)
    NoteNotFound,
    InvalidToken,
    NoSpecifiedContent,
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NoteCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub content: serde_json::Value,
}
impl NoteCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Note, NoteError> {
        info!("creating note: {:?}", self);
        Ok(diesel::insert_into(notes::table)
            .values(self)
            .get_result(conn)
            .map_err(NoteError::DatabaseError)?)
    }
}

#[derive(AsChangeset, Debug)]
#[table_name = "notes"]
pub struct NoteUpdateSpec {
    pub updated_at: DateTime<Utc>,
    pub content: serde_json::Value,
}
impl NoteUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<Note, NoteError> {
        info!("updating note {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_notes
                .filter(notes::api_id.eq(&api_id))
                .filter(notes::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<Note>(conn)
        .map_err(NoteError::DatabaseError)?)
    }
}

impl Note {
    pub fn find_all_for_user(conn: &PgConnection, token: String) -> Result<Vec<Note>, NoteError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| NoteError::NoteNotFound)?;
        let notes: Vec<Note> = all_notes
            .filter(notes::user_id.eq(session.user_id))
            .load(conn)
            .map_err(|_| NoteError::NoteNotFound)?;
        Ok(notes)
    }

    pub fn create(
        conn: &PgConnection,
        token: String,
        content: serde_json::Value,
    ) -> Result<Note, NoteError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| NoteError::InvalidToken)?;
        NoteCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Note),
            user_id: session.user_id,
            content,
        }
        .insert(conn)
    }

    // TODO: move this out of the model.
    //    (models/note.rs should not handle API spec conversion)
    pub fn update(
        conn: &PgConnection,
        token: String,
        api_id: String,
        spec: ApiNoteUpdateSpec,
    ) -> Result<Note, NoteError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| NoteError::InvalidToken)?;
        NoteUpdateSpec {
            updated_at: Utc::now(),
            content: spec.content,
        }
        .update(conn, api_id, session.user_id)
    }
}
