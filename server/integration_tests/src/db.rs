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

    let conn = get_conn(&pool).unwrap();
    println!("🌱️ Connecting to test database...\n");
    embedded_migrations::run_with_output(&conn, &mut io::stdout().lock()).unwrap();
    conn.execute("BEGIN").unwrap();

    pool
}

pub fn rollback(pool: &PgPool) {
    let conn = get_conn(&pool).unwrap();
    println!("\n🌬 Rolling back...");
    conn.execute("ROLLBACK").unwrap();
}

#[allow(unused)]
pub fn destroy_test_db(pool: &PgPool) {
    let conn = get_conn(&pool).unwrap();
    println!("🪓 Destroying test database...");
    conn.execute("ROLLBACK").unwrap();
    conn.execute("DROP TABLE tasks").unwrap();
    conn.execute("DROP TABLE sessions").unwrap();
    conn.execute("DROP TABLE users").unwrap();
    conn.execute("DROP TABLE __diesel_schema_migrations")
        .unwrap();
    conn.execute("COMMIT").unwrap();
}
