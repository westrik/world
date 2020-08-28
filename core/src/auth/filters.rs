use warp::Filter;

use crate::auth::handlers;
use crate::auth::handlers::{CloudfrontAuthenticationRequest, SignInRequest};
use crate::auth::models::user::ApiUserCreateSpec;
use crate::db::DbPool;
use crate::routes::{json_body, with_db, with_session};

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sign_up(db_pool.clone()).or(sign_in(db_pool))
}

/// POST /sign_up with JSON body
pub fn sign_up(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-up")
        .and(warp::post())
        .and(json_body::<ApiUserCreateSpec>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_up)
}

/// POST /sign-in with JSON body
pub fn sign_in(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-in")
        .and(warp::post())
        .and(json_body::<SignInRequest>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_in)
}

/// POST /authenticate:cloudfront with JSON body
pub fn authenticate_cloudfront(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("authenticate:cloudfront")
        .and(warp::post())
        .and(json_body::<CloudfrontAuthenticationRequest>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::cloudfront_authenticate)
}
