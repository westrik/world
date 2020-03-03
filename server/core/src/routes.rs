use crate::auth::filters::routes as auth_routes;
use crate::db::DbPool;
use crate::notes::filters::routes as note_routes;
use crate::tasks::filters::routes as task_routes;
use crate::API_VERSION;
use warp::cors::Cors;
use warp::Filter;

pub fn api(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    preflight_cors()
        .or(health_check())
        .or(authentication(db_pool.clone()))
        .or(authenticated(db_pool))
        .map(|r| warp::reply::with_header(r, "x-api-version", API_VERSION))
}

fn authentication(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    auth_routes(db_pool)
}

fn authenticated(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // TODO: wrap with session
    task_routes(db_pool.clone()).or(note_routes(db_pool))
}

//fn admin_authenticated(
//    _db_pool: PgPool,
//) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//    unimplemented!()
//}

pub fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_session_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("authorization"))
        .map(|token: String| token)
}

pub fn json_body<T: Send + serde::de::DeserializeOwned>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json::<T>())
}

pub fn preflight_cors() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::options().map(warp::reply)
}

pub fn health_check() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| Ok(format!("OK, version {}", API_VERSION)))
}

pub fn cors_wrapper(cors_origin_url: &str) -> Cors {
    warp::cors()
        .allow_origin(cors_origin_url)
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .build()
}
