use warp::Filter;
use westrikworld_core::db::DbPool;
use westrikworld_core::routes::{health_check, preflight_cors};
use westrikworld_core::API_VERSION;

pub fn worker_api(
    _db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    preflight_cors()
        .or(health_check("worker"))
        .map(|r| warp::reply::with_header(r, "X-API-Version", API_VERSION))
}
