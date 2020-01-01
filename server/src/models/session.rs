use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::user::{User, UserQueryError};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(token)]
#[belongs_to(User)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn create(conn: &PgConnection, user: &User) -> Result<Session, UserQueryError> {
        let token: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        // TODO: retry on token collision
        Ok(diesel::insert_into(sessions::table)
            .values((sessions::token.eq(token), sessions::user_id.eq(user.id)))
            .get_result(conn)
            .map_err(|err| UserQueryError::DatabaseError(err))?)
    }
}

#[derive(Serialize)]
pub struct UiSession {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl From<Session> for UiSession {
    fn from(session: Session) -> Self {
        UiSession {
            token: session.token,
            expires_at: session.expires_at,
        }
    }
}
