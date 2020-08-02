use argon2rs::verifier::Encoded;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use std::{env, str};

use crate::errors::ApiError;
use crate::schema::{users, users::dsl::users as all_users};

/* ----- Model definitions -----  */

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(skip)]
    pub password_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
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
    pub fn insert(&self, conn: &PgConnection) -> Result<User, ApiError> {
        Ok(diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}
#[derive(Deserialize)]
pub struct ApiUserCreateSpec {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub password: String,
}

lazy_static! {
    static ref HASH_SALT: String =
        env::var("PASSWORD_HASH_SALT").expect("PASSWORD_HASH_SALT must be set");
}

impl User {
    pub fn create(new_user: ApiUserCreateSpec, conn: &PgConnection) -> Result<User, ApiError> {
        let new_user = UserCreateSpec {
            email_address: new_user.email_address.to_lowercase(),
            full_name: new_user.full_name,
            password_hash: Self::hash_password(new_user.password),
        };
        new_user.insert(conn)
    }

    pub fn find(
        email_address: &str,
        password: &str,
        conn: &PgConnection,
    ) -> Result<User, ApiError> {
        let user: User = all_users
            .filter(users::email_address.eq(email_address.to_lowercase()))
            .filter(users::password_hash.eq(Self::hash_password(password.to_string())))
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::Forbidden,
                _ => ApiError::DatabaseError(e),
            })?;
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
