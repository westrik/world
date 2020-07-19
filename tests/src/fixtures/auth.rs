use diesel::prelude::*;
use diesel::PgConnection;
use world_core::auth::models::session::Session;
use world_core::auth::models::user::{ApiUserCreateSpec, User};
use world_core::schema::{sessions, sessions::dsl::sessions as all_sessions};
use world_core::schema::{users, users::dsl::users as all_users};

pub const TEST_USER_EMAIL_ADDRESS: &str = "dolores@westrik.world";
pub const TEST_USER_FULL_NAME: &str = "Dolores Abernathy";
pub const TEST_USER_PASSWORD: &str = "password123";

pub fn create_test_user(conn: &PgConnection) {
    println!("🤖 creating test user");
    User::create(
        ApiUserCreateSpec {
            email_address: TEST_USER_EMAIL_ADDRESS.to_string(),
            full_name: Some(TEST_USER_FULL_NAME.to_string()),
            password: TEST_USER_PASSWORD.to_string(),
        },
        conn,
    )
    .unwrap();
}

pub fn create_test_session(conn: &PgConnection) {
    let user = get_test_user(&conn);
    println!("🔑 creating test session");
    Session::create(conn, &user).unwrap();
}

pub fn get_test_user(conn: &PgConnection) -> User {
    all_users
        .filter(users::email_address.eq(TEST_USER_EMAIL_ADDRESS))
        .first(conn)
        .unwrap()
}

pub fn get_test_session(conn: &PgConnection, user_id: i32) -> Session {
    all_sessions
        .filter(sessions::user_id.eq(user_id))
        .first(conn)
        .unwrap()
}
