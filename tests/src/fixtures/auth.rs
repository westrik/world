use diesel::prelude::*;
use diesel::PgConnection;
use world_core::auth::models::session::Session;
use world_core::auth::models::user::{ApiUserCreateSpec, User};
use world_core::schema::{sessions, sessions::dsl::sessions as all_sessions};
use world_core::schema::{users, users::dsl::users as all_users};

pub const TEST_USER_EMAIL_ADDRESS: &str = "dolores@westrik.world";
pub const TEST_USER_FULL_NAME: &str = "Dolores Abernathy";
pub const TEST_USER_PASSWORD: &str = "password123";

pub const TEST_USER2_EMAIL_ADDRESS: &str = "hÉLLOwOrLd@WESTRIK.WORLD";
pub const TEST_USER2_FULL_NAME: &str = "ﾟ･✿ヾ╲(｡◕‿◕｡)╱✿･ﾟ";
pub const TEST_USER2_PASSWORD: &str = "dB7TR2)X/4X$YnRRAo4^2M3ETC4QpVAUn]dTtUb+q9majn>3T$Umt{dLL[jT2Xovi3rqWCy/9[(b/9nefi=W@uuv2pdL+Z7RyZMPfd4iAWEw28XyK?Mtu3FUU2,xsV^y";

pub fn create_test_users(conn: &PgConnection) {
    println!("🤖 creating test users");
    User::create(
        ApiUserCreateSpec {
            email_address: TEST_USER_EMAIL_ADDRESS.to_string(),
            full_name: Some(TEST_USER_FULL_NAME.to_string()),
            password: TEST_USER_PASSWORD.to_string(),
        },
        conn,
    )
    .unwrap();
    User::create(
        ApiUserCreateSpec {
            email_address: TEST_USER2_EMAIL_ADDRESS.to_string(),
            full_name: Some(TEST_USER2_FULL_NAME.to_string()),
            password: TEST_USER2_PASSWORD.to_string(),
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
