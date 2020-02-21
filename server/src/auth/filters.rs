use crate::auth::handlers;
use crate::auth::handlers::SignInRequest;
use crate::auth::models::user::ApiUserCreateSpec;
use crate::db::PgPool;
use crate::routes::utils::{json_body, with_db};
use warp::Filter;

pub fn routes(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sign_up(db_pool.clone()).or(sign_in(db_pool))
}

/// POST /sign_up with JSON body
pub fn sign_up(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-up")
        .and(warp::post())
        .and(json_body::<ApiUserCreateSpec>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_up)
}

/// POST /sign-in with JSON body
pub fn sign_in(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-in")
        .and(warp::post())
        .and(json_body::<SignInRequest>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_in)
}
