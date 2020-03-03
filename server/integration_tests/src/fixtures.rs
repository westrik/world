use diesel::prelude::*;
use diesel::PgConnection;
use westrikworld_core::auth::models::session::Session;
use westrikworld_core::auth::models::user::{ApiUserCreateSpec, User};
use westrikworld_core::schema::{sessions, sessions::dsl::sessions as all_sessions};
use westrikworld_core::schema::{users, users::dsl::users as all_users};

const TEST_EMAIL_ADDRESS: &str = "testuser@example.com";

pub fn create_test_user(conn: &PgConnection) {
    println!("🤖 creating test user");
    User::create(
        ApiUserCreateSpec {
            emailAddress: "testuser@example.com".to_string(),
            fullName: Some("Test User".to_string()),
            password: "password".to_string(),
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
        .filter(users::email_address.eq(TEST_EMAIL_ADDRESS))
        .first(conn)
        .unwrap()
}

pub fn get_test_session(conn: &PgConnection, user_id: i32) -> Session {
    all_sessions
        .filter(sessions::user_id.eq(user_id))
        .first(conn)
        .unwrap()
}
