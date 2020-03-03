use crate::db::DbPool;
use crate::routes::{json_body, with_db, with_session_token};
use crate::tasks::handlers;
use crate::tasks::models::task::{ApiTaskCreateSpec, ApiTaskUpdateSpec};
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_list(db_pool.clone())
        .or(tasks_create(db_pool.clone()))
        .or(tasks_update(db_pool.clone()))
        .or(tasks_delete(db_pool))
}

/// GET /task?offset=3&limit=5
pub fn tasks_list(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::list_tasks)
}

/// POST /task with JSON body
pub fn tasks_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task")
        .and(warp::post())
        .and(json_body::<ApiTaskCreateSpec>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::create_task)
}

/// PUT /task/:id with JSON body
pub fn tasks_update(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task" / String)
        .and(warp::put())
        .and(json_body::<ApiTaskUpdateSpec>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::update_task)
}

/// DELETE /task/:id
pub fn tasks_delete(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task" / String)
        .and(warp::delete())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::delete_task)
}
