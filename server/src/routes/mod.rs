use crate::db::PgPool;
use crate::tasks::filters::routes as tasks_routes;
use warp::http::StatusCode;
use warp::Filter;

pub fn api(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authentication(db_pool.clone())
        .or(authenticated(db_pool.clone()))
        .or(admin_authenticated(db_pool))
}

fn authentication(
    _db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-in")
        .and(warp::post())
        .map(|| Ok(StatusCode::OK))
}

fn authenticated(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_routes(db_pool.clone())
}

fn admin_authenticated(
    _db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::get())
        .map(|| Ok(StatusCode::OK))
}
