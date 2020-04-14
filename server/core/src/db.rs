use crate::errors::ApiError;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str, max_connections: u32) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(max_connections).build(manager)
}

pub fn get_conn(pool: &DbPool) -> Result<DbPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn begin_txn(conn: &PgConnection) -> Result<usize, ApiError> {
    conn.execute("BEGIN").map_err(ApiError::DatabaseError)
}

pub fn commit_txn(conn: &PgConnection) -> Result<usize, ApiError> {
    conn.execute("COMMIT").map_err(ApiError::DatabaseError)
}

pub fn rollback_txn(conn: &PgConnection) -> Result<usize, ApiError> {
    conn.execute("ROLLBACK").map_err(ApiError::DatabaseError)
}
