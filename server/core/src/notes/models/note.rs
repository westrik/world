use chrono::{DateTime, Utc};

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::notes::content::schema::Content;
use crate::notes::errors::NoteError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{notes, notes::dsl::notes as all_notes};
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
    pub content: Option<serde_json::Value>,
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
    pub fn find_all_for_user(
        conn: &PgConnection,
        session: Session,
    ) -> Result<Vec<Note>, NoteError> {
        let notes: Vec<Note> = all_notes
            .filter(notes::user_id.eq(session.user_id))
            .load(conn)
            .map_err(|_| NoteError::NoteNotFound)?;
        Ok(notes)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        content: serde_json::Value,
    ) -> Result<Note, NoteError> {
        NoteCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Note),
            user_id: session.user_id,
            content,
        }
        .insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        content: Option<Content>,
    ) -> Result<Note, NoteError> {
        let mut content_update = None;
        if let Some(content_data) = content {
            content_update = Some(
                serde_json::to_value(content_data).map_err(|_| NoteError::BadContentConversion)?,
            );
        }
        NoteUpdateSpec {
            updated_at: Utc::now(),
            content: content_update,
        }
        .update(conn, api_id, session.user_id)
    }
}
