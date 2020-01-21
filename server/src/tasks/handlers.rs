use crate::db::{get_conn, PgPool};
use crate::models::task::{ListOptions, Task, TaskQueryError};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
pub struct GetItemResponse {
    error: Option<String>,
    items: Option<Vec<Task>>,
}

fn run_get_tasks(token: String, pool: &PgPool) -> Result<Vec<Task>, TaskQueryError> {
    Ok(Task::find_all_for_user(&get_conn(&pool).unwrap(), token)?)
}

pub async fn list_tasks(
    opts: ListOptions,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("list_tasks: token={}, opts={:?}", session_token, opts);
    Ok(match run_get_tasks(session_token, &db_pool) {
        Ok(items) => warp::reply::with_status(
            warp::reply::json(&GetItemResponse {
                error: None,
                items: Some(items),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetItemResponse {
                error: Some("Failed to query for items".to_string()),
                items: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub content: String,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    error: Option<String>,
    item: Option<Task>,
}

fn run_create_task(token: String, content: String, pool: &PgPool) -> Result<Task, TaskQueryError> {
    Ok(Task::create(&get_conn(&pool).unwrap(), token, content)?)
}

pub async fn create_task(
    new_task: NewTask,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "create_task: token={}, new_task={:?}",
        session_token, new_task
    );
    Ok(
        match run_create_task(session_token, new_task.content, &db_pool) {
            Ok(item) => warp::reply::with_status(
                warp::reply::json(&CreateTaskResponse {
                    error: None,
                    item: Some(item),
                }),
                StatusCode::OK,
            ),
            Err(_) => warp::reply::with_status(
                warp::reply::json(&CreateTaskResponse {
                    error: Some("Failed to query for items".to_string()),
                    item: None,
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        },
    )
}

pub async fn update_task(
    id: u64,
    task_update: NewTask,
    session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "update_task: token={}, id={}, task_update={:?}",
        session_token, id, task_update
    );
    Ok(StatusCode::OK)
}

pub async fn delete_task(
    id: u64,
    session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_task: token={}, id={}", session_token, id);
    Ok(StatusCode::NO_CONTENT)
}
