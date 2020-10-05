use diesel::dsl::now;
use diesel::prelude::*;
use serde_json::json;
use std::convert::Infallible;
use warp::cors::Cors;
use warp::Filter;

use crate::auth::filters::{authenticate_cloudfront, routes as auth_routes};
use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::{handle_rejection, ApiError};
use crate::library::filters::routes as library_routes;
use crate::notes::filters::routes as note_routes;
use crate::settings::filters::routes as settings_routes;
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::tasks::filters::routes as task_routes;
use crate::utils::api_task::run_api_task;
use crate::{API_VERSION, MAX_CONTENT_LENGTH_BYTES};

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
    authenticate_cloudfront(db_pool.clone())
        .or(task_routes(db_pool.clone()))
        .or(note_routes(db_pool.clone()))
        .or(library_routes(db_pool.clone()))
        .or(settings_routes(db_pool))
}

// Helpers:

pub fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

async fn load_session_for_token(token: String, db_pool: DbPool) -> Result<Session, ApiError> {
    Ok(run_api_task(move || {
        all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(&get_conn(&db_pool).unwrap())
            .map_err(ApiError::DatabaseError)
    })
    .await?)
}

pub fn with_session(
    db_pool: DbPool,
) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("Authorization"))
        .and(with_db(db_pool))
        .and_then(|token: String, db_pool: DbPool| async move {
            load_session_for_token(token, db_pool)
                .await
                .map_err(warp::reject::custom)
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
        .allow_credentials(true)
        .build()
}
