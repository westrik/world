use argon2rs::verifier::Encoded;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use std::{env, str};

use crate::schema::{users, users::dsl::users as all_users};

/* ----- Model definitions -----  */

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email_address: String,
    pub full_name: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/* ----- Query helper structs  -----  */

#[derive(Debug)]
pub enum UserQueryError {
    UserNotFound,
    DatabaseError(diesel::result::Error),
}

/* ----- Create and update specs  -----  */

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserCreateSpec {
    pub email_address: String,
    pub full_name: Option<String>,
    pub password_hash: String,
}
impl UserCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<User, UserQueryError> {
        Ok(diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
            .map_err(UserQueryError::DatabaseError)?)
    }
}
#[derive(Debug, Deserialize)]
pub struct ApiUserCreateSpec {
    pub email_address: String,
    pub full_name: Option<String>,
    pub password: String,
}

/* ----- API interfaces -----  */

#[derive(Serialize)]
pub struct ApiUser {
    pub email_address: String, // TODO: change to user API key
    pub full_name: Option<String>,
}

impl From<User> for ApiUser {
    fn from(user: User) -> Self {
        ApiUser {
            email_address: user.email_address,
            full_name: user.full_name,
        }
    }
}

/* ----- DB business logic -----  */

lazy_static! {
    static ref HASH_SALT: String =
        env::var("PASSWORD_HASH_SALT").expect("PASSWORD_HASH_SALT must be set");
}

impl User {
    pub fn create(
        new_user: ApiUserCreateSpec,
        conn: &PgConnection,
    ) -> Result<User, UserQueryError> {
        let new_user = UserCreateSpec {
            email_address: new_user.email_address,
            full_name: new_user.full_name,
            password_hash: Self::hash_password(new_user.password),
        };
        new_user.insert(conn)
    }

    pub fn find(
        email_address: &str,
        password: &str,
        conn: &PgConnection,
    ) -> Result<User, UserQueryError> {
        let user: User = all_users
            .filter(users::email_address.eq(email_address))
            .filter(users::password_hash.eq(Self::hash_password(password.to_string())))
            .first(conn)
            .map_err(|_| UserQueryError::UserNotFound)?;
        Ok(user)
    }

    pub fn delete_for_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_users.find(id)).execute(conn)
    }

    pub fn hash_password(password: String) -> String {
        str::from_utf8(
            &Encoded::default2i(password.as_ref(), HASH_SALT.as_ref(), b"key", b"").to_u8(),
        )
        .unwrap()
        .to_string()
    }
}

/* ----- TODO: DB integration tests -----  */
