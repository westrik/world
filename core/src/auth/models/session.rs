use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::schema::sessions;
use crate::schema::{users, users::dsl::users as all_users};

#[derive(Associations, Clone, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[primary_key(token)]
#[belongs_to(User)]
pub struct Session {
    #[serde(skip)]
    pub user_id: i32,
    pub token: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn create(conn: &PgConnection, user: &User) -> Result<Session, ApiError> {
        let token: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        // TODO: retry on token collision
        Ok(diesel::insert_into(sessions::table)
            .values((sessions::token.eq(token), sessions::user_id.eq(user.id)))
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn get_user(&self, conn: &PgConnection) -> Result<User, ApiError> {
        Ok(all_users
            .filter(users::id.eq(self.user_id))
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    ApiError::InternalError("Session is invalid".to_string())
                }
                _ => ApiError::DatabaseError(e),
            })?)
    }
}
