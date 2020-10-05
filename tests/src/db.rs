use diesel::Connection;
use dotenv::dotenv;
use std::{env, io};

pub use world_core::db::{
    begin_txn, commit_txn, get_conn, init_pool, rollback_txn, DbPool,
    DbPooledConnection as DbConnection,
};

use crate::fixtures::*;

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 20;

pub fn create_test_db() -> DbPool {
    dotenv().ok();
    let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = init_pool(&test_database_url, DB_POOL_SIZE).expect("Failed to create pool");

    destroy_test_db(&pool);

    println!("🌱️ running all migrations...");
    let conn = get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut io::stdout().lock())
        .expect("running migrations failed");
    create_test_users(&conn);
    create_test_session(&conn);

    pool
}

pub fn destroy_test_db(pool: &DbPool) {
    let conn = get_conn(&pool).unwrap();
    println!("🪓 destroying test database...");
    rollback_txn(&conn).unwrap();
    // TODO: automatically drop tables in the right order
    conn.execute("DROP TABLE IF EXISTS site_pages").unwrap();
    conn.execute("DROP TABLE IF EXISTS sites").unwrap();
    conn.execute("DROP TABLE IF EXISTS library_item_versions")
        .unwrap();
    conn.execute("DROP TABLE IF EXISTS library_items").unwrap();
    conn.execute("DROP TABLE IF EXISTS tasks").unwrap();
    conn.execute("DROP TABLE IF EXISTS note_versions").unwrap();
    conn.execute("DROP TABLE IF EXISTS notes").unwrap();
    conn.execute("DROP TABLE IF EXISTS sessions").unwrap();
    conn.execute("DROP TABLE IF EXISTS jobs").unwrap();
    conn.execute("DROP TABLE IF EXISTS users").unwrap();
    conn.execute("DROP TABLE IF EXISTS __diesel_schema_migrations")
        .unwrap();
    commit_txn(&conn).unwrap();
}
