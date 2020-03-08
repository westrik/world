use crate::auth::filters::routes as auth_routes;
use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::notes::filters::routes as note_routes;
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::tasks::filters::routes as task_routes;
use crate::{API_VERSION, MAX_CONTENT_LENGTH_BYTES};
use diesel::dsl::now;
use diesel::prelude::*;
use warp::cors::Cors;
use warp::Filter;

#[derive(Debug, Serialize)]
pub enum GenericError {
    Unauthorized,
}
impl warp::reject::Reject for GenericError {}

pub fn api(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    preflight_cors()
        .or(health_check())
        .or(authentication(db_pool.clone()))
        .or(authenticated(db_pool))
        .map(|r| warp::reply::with_header(r, "X-API-Version", API_VERSION))
}

fn authentication(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    auth_routes(db_pool)
}

fn authenticated(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    task_routes(db_pool.clone()).or(note_routes(db_pool))
}

// Helpers:

pub fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn load_session_for_token(
    token: String,
    db_pool: DbPool,
) -> Result<Session, diesel::result::Error> {
    // TODO: move to query thread pool
    all_sessions
        .filter(sessions::token.eq(token))
        .filter(sessions::expires_at.gt(now))
        .first(&get_conn(&db_pool).unwrap())
}

pub fn with_session(
    db_pool: DbPool,
) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("Authorization"))
        .and(with_db(db_pool))
        .and_then(|token: String, db_pool: DbPool| async move {
            load_session_for_token(token, db_pool).map_err(|_| warp::reject())
        })
}

pub fn json_body<T: Send + serde::de::DeserializeOwned>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_CONTENT_LENGTH_BYTES).and(warp::body::json::<T>())
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
