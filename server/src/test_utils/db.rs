pub use crate::db::PgPool;
pub use crate::db::{get_conn, init_pool};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::Connection;
use dotenv::dotenv;
use std::env;

embed_migrations!();

pub fn spin_up_test_database() -> PgPool {
    dotenv().ok();
    let test_database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = init_pool(&test_database_url).expect("Failed to create pool");

    let conn = get_conn(&pool).unwrap();
    println!("Creating test database");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    pool
}

pub fn destroy_test_database(pool: &PgPool) {
    let conn = get_conn(&pool).unwrap();
    println!("Destroying test database");
    conn.execute("DROP TABLE tasks");
    conn.execute("DROP TABLE sessions");
    conn.execute("DROP TABLE users");
    conn.execute("DROP TABLE __diesel_schema_migrations");
}
