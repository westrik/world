use crate::auth::models::user::{ApiUserCreateSpec, User};
use diesel::PgConnection;

pub fn create_test_user(conn: &PgConnection) -> User {
    println!("🤖 Creating test user");
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
