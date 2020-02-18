use crate::auth::models::user::{NewUser, User};
use diesel::PgConnection;

pub fn create_test_user(conn: &PgConnection) -> User {
    println!("👩‍💻 Creating test user");
    User::create(
        NewUser {
            email_address: "testuser@example.com".to_string(),
            full_name: Some("Test User".to_string()),
            password: "password".to_string(),
        },
        conn,
    )
    .unwrap()
}
