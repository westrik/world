use warp::Filter;
use world_core::db::DbPool;
use world_core::routes::{health_check, preflight_cors};
use world_core::API_VERSION;

pub fn worker_api(
    _db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    preflight_cors()
        .or(health_check("worker"))
        .map(|r| warp::reply::with_header(r, "X-API-Version", API_VERSION))
}
