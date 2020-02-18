pub use crate::db::PgPool;
pub use crate::db::{get_conn, init_pool};
use diesel::Connection;
use dotenv::dotenv;
use std::{env, io};

embed_migrations!();

pub fn spin_up_test_database() -> PgPool {
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

pub fn destroy(pool: &PgPool) {
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
