use diesel::pg::PgConnection;
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
