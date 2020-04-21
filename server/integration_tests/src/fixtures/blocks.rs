use diesel::PgConnection;

use westrikworld_core::content::models::block::Block;

use crate::fixtures::*;

pub fn create_test_block(conn: &PgConnection, user_id: i32) -> Block {
    let session = get_test_session(conn, user_id);
    Block::create(conn, session, None, None).unwrap()
}
