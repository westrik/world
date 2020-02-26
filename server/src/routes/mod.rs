use crate::auth::filters::routes as auth_routes;
use crate::db::PgPool;
use crate::notes::filters::routes as note_routes;
use crate::routes::utils::{health_check, preflight_cors};
use crate::tasks::filters::routes as task_routes;
use crate::API_VERSION;
use warp::Filter;

pub mod options;
pub mod utils;

pub fn api(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    preflight_cors()
        .or(health_check())
        .or(authentication(db_pool.clone()))
        .or(authenticated(db_pool))
        .map(|r| warp::reply::with_header(r, "x-api-version", API_VERSION))
}

fn authentication(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    auth_routes(db_pool)
}

fn authenticated(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // TODO: wrap with session
    task_routes(db_pool.clone()).or(note_routes(db_pool))
}

//fn admin_authenticated(
//    _db_pool: PgPool,
//) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//    unimplemented!()
//}
