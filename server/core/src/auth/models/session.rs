use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::auth::errors::UserError;
use crate::auth::models::user::User;
use crate::schema::sessions;

/* ----- Model definitions -----  */

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[primary_key(token)]
#[belongs_to(User)]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/* ----- API interfaces -----  */

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ApiSession {
    pub token: String,
    pub expiresAt: DateTime<Utc>,
}

impl From<Session> for ApiSession {
    fn from(session: Session) -> Self {
        ApiSession {
            token: session.token,
            expiresAt: session.expires_at,
        }
    }
}

/* ----- DB business logic -----  */

impl Session {
    pub fn create(conn: &PgConnection, user: &User) -> Result<Session, UserError> {
        let token: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        // TODO: retry on token collision
        Ok(diesel::insert_into(sessions::table)
            .values((sessions::token.eq(token), sessions::user_id.eq(user.id)))
            .get_result(conn)
            .map_err(UserError::DatabaseError)?)
    }
}
