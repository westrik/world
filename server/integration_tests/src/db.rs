use diesel::Connection;
use dotenv::dotenv;
use std::{env, io};
pub use westrikworld_core::db::PgPool;
pub use westrikworld_core::db::{get_conn, init_pool};

embed_migrations!("../core/migrations");

pub fn connect_to_test_db() -> PgPool {
    dotenv().ok();
    let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = init_pool(&test_database_url).expect("Failed to create pool");
    destroy_test_db(&pool);

    println!("🌱️ running all migrations...");
    let conn = get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut io::stdout().lock()).unwrap();
    print!("\n");

    pool
}

pub fn start_txn(pool: &PgPool) {
    let conn = get_conn(&pool).unwrap();

    println!("📋 starting transaction");
    conn.execute("BEGIN").unwrap();
}

pub fn rollback_txn(pool: &PgPool) {
    let conn = get_conn(&pool).unwrap();

    println!("🧻 rolling back...");
    conn.execute("ROLLBACK").unwrap();
}

#[allow(unused)]
pub fn destroy_test_db(pool: &PgPool) {
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
