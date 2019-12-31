use crate::schema::{users, users::dsl::users as all_users};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email_address: String,
    pub full_name: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct DbNewUser {
    pub email_address: String,
    pub full_name: Option<String>,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email_address: String,
    pub full_name: Option<String>,
    pub password: String,
}

#[derive(Debug)]
pub enum UserQueryError {
    IncorrectPassword,
    UnknownEmailAddress,
    DatabaseError(diesel::result::Error),
}

impl User {
    pub fn create(new_user: NewUser, conn: &PgConnection) -> Result<User, UserQueryError> {
        let new_user = DbNewUser {
            email_address: new_user.email_address,
            full_name: new_user.full_name,
            password_hash: new_user.password,
        };
        new_user.insert(conn)
    }

    //    pub fn find_user(email_address: String, password: String) -> Result<User, UserQueryError> {
    //    }

    pub fn delete_for_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_users.find(id)).execute(conn)
    }

    pub fn delete_all(conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_users).execute(conn)
    }
}

impl DbNewUser {
    pub fn insert(&self, conn: &PgConnection) -> Result<User, UserQueryError> {
        let res = diesel::insert_into(users::table)
            .values(self)
            .get_result(conn);
        match res {
            Ok(new_user) => Ok(new_user),
            Err(oops) => Err(UserQueryError::DatabaseError(oops)),
        }
    }
}
