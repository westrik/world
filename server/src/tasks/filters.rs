use crate::db::PgPool;
use crate::models::item::Item;
use crate::models::item::ListOptions;
use crate::tasks::handlers;
use warp::Filter;

pub fn routes(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_list(db_pool.clone())
        .or(tasks_create(db_pool.clone()))
        .or(tasks_update(db_pool.clone()))
        .or(tasks_delete(db_pool))
}

/// GET /tasks?offset=3&limit=5
pub fn tasks_list(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::list_tasks)
}

/// POST /tasks with JSON body
pub fn tasks_create(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks")
        .and(warp::post())
        .and(json_body())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::create_task)
}

/// PUT /tasks/:id with JSON body
pub fn tasks_update(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tasks" / u64)
        .and(warp::put())
        .and(json_body())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::update_task)
}

/// DELETE /tasks/:id
pub fn tasks_delete(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // We'll make one of our endpoints admin-only to show how authentication filters are used
    warp::path!("tasks" / u64)
        .and(warp::delete())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::delete_task)
}

fn with_db(
    db_pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_session_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("authorization"))
        .map(|token: String| token)
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // We want a JSON body, and to reject huge payloads
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
