use diesel::Connection;
use dotenv::dotenv;
use std::{env, io};
pub use westrikworld_core::db::{get_conn, init_pool, DbPool};

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 20;

pub fn connect_to_test_db() -> DbPool {
    dotenv().ok();
    let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = init_pool(&test_database_url, DB_POOL_SIZE).expect("Failed to create pool");

    destroy_test_db(&pool);

    println!("🌱️ running all migrations...");
    let conn = get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut io::stdout().lock()).unwrap();

    pool
}

pub fn start_txn(pool: &DbPool) {
    get_conn(&pool).unwrap().execute("BEGIN").unwrap();
}

pub fn rollback_txn(pool: &DbPool) {
    get_conn(&pool).unwrap().execute("ROLLBACK").unwrap();
}

#[allow(unused)]
pub fn destroy_test_db(pool: &DbPool) {
    let conn = get_conn(&pool).unwrap();
    println!("🪓 destroying test database...");
    conn.execute("ROLLBACK").unwrap();
    conn.execute("DROP TABLE IF EXISTS notes").unwrap();
    conn.execute("DROP TABLE IF EXISTS tasks").unwrap();
    conn.execute("DROP TABLE IF EXISTS sessions").unwrap();
    conn.execute("DROP TABLE IF EXISTS users").unwrap();
    conn.execute("DROP TABLE IF EXISTS __diesel_schema_migrations")
        .unwrap();
    conn.execute("COMMIT").unwrap();
}
