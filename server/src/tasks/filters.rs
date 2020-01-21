use crate::db::PgPool;
use crate::models::task::ListOptions;
use crate::routes::utils::{json_body, with_db, with_session_token};
use crate::tasks::handlers;
use crate::tasks::handlers::NewTask;
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
    warp::path!("task")
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
    warp::path!("task")
        .and(warp::post())
        .and(json_body::<NewTask>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::create_task)
}

/// PUT /tasks/:id with JSON body
pub fn tasks_update(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task" / u64)
        .and(warp::put())
        .and(json_body::<NewTask>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::update_task)
}

/// DELETE /tasks/:id
pub fn tasks_delete(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("task" / u64)
        .and(warp::delete())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::delete_task)
}
