use crate::auth::filters::routes as auth_routes;
use crate::db::PgPool;
use crate::tasks::filters::routes as task_routes;
use warp::Filter;

pub mod utils;

pub fn api(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authentication(db_pool.clone()).or(authenticated(db_pool))
    //        .or(admin_authenticated(db_pool))
}

fn authentication(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    auth_routes(db_pool)
}

fn authenticated(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    task_routes(db_pool)
}

//fn admin_authenticated(
//    _db_pool: PgPool,
//) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//    warp::path!("user")
//        .and(warp::get())
//        .map(|| Ok(StatusCode::OK))
//}
