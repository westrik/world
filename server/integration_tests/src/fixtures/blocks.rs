use diesel::prelude::*;
use diesel::PgConnection;
use westrikworld_core::auth::models::session::Session;
use westrikworld_core::auth::models::user::{ApiUserCreateSpec, User};
use westrikworld_core::schema::{sessions, sessions::dsl::sessions as all_sessions};
use westrikworld_core::schema::{users, users::dsl::users as all_users};

pub fn create_test_block(conn: &PgConnection, user_id: i32) {
    println!("🅰 creating test block");
    Block::create(
        BlockCreateSpec {
            user_id,
        },
        conn,
    )
    .unwrap()
}
