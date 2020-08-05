use diesel::dsl::now;
use diesel::prelude::*;
use diesel::result::Error;
use serde_json::json;
use std::convert::Infallible;
use warp::cors::Cors;
use warp::Filter;

use crate::auth::filters::routes as auth_routes;
use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::{handle_rejection, ApiError};
use crate::library::filters::routes as library_routes;
use crate::notes::filters::routes as note_routes;
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::tasks::filters::routes as task_routes;
use crate::{API_VERSION, MAX_CONTENT_LENGTH_BYTES};
use tokio::task::block_in_place;

pub fn api(db_pool: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    preflight_cors()
        .or(health_check("api"))
        .or(authentication(db_pool.clone()))
        .or(authenticated(db_pool))
        .map(|r| warp::reply::with_header(r, "X-API-Version", API_VERSION))
        .recover(handle_rejection)
}

fn authentication(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    auth_routes(db_pool)
}

fn authenticated(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    task_routes(db_pool.clone())
        .or(note_routes(db_pool.clone()))
        .or(library_routes(db_pool))
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
    // TODO: get rid of .unwrap()'s somehow
    block_in_place(move || {
        all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(&get_conn(&db_pool).unwrap())
    })
}

pub fn with_session(
    db_pool: DbPool,
) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("Authorization"))
        .and(with_db(db_pool))
        .and_then(|token: String, db_pool: DbPool| async move {
            load_session_for_token(token, db_pool).map_err(|e| {
                warp::reject::custom(match e {
                    Error::NotFound => ApiError::Forbidden,
                    e => ApiError::DatabaseError(e),
                })
            })
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

pub fn health_check(
    service_name: &'static str,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'static {
    let response = json!({
        "service": service_name,
        "status": "ok",
        "version": API_VERSION
    });
    warp::path::end().map(move || warp::reply::json(&response))
}

pub fn cors_wrapper(cors_origin_url: &str) -> Cors {
    warp::cors()
        .allow_origin(cors_origin_url)
        .allow_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .build()
}
