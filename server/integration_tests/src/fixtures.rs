use diesel::PgConnection;
use westrikworld_core::auth::models::user::{ApiUserCreateSpec, User};

pub fn create_test_user(conn: &PgConnection) -> User {
    println!("🤖 creating test user");
    User::create(
        ApiUserCreateSpec {
            emailAddress: "testuser@example.com".to_string(),
            fullName: Some("Test User".to_string()),
            password: "password".to_string(),
        },
        conn,
    )
    .unwrap()
}
