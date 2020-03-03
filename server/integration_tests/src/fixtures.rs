use diesel::prelude::*;
use diesel::PgConnection;
use westrikworld_core::auth::models::user::{ApiUserCreateSpec, User};
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

pub fn get_test_user(conn: &PgConnection) -> User {
    all_users
        .filter(users::email_address.eq(TEST_EMAIL_ADDRESS))
        .first(conn)
        .unwrap()
}
