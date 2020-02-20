use crate::db::{get_conn, PgPool};
use crate::tasks::model::{ApiNewTask, ListOptions, Task, TaskQueryError};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
pub struct GetTaskResponse {
    error: Option<String>,
    tasks: Option<Vec<Task>>,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    error: Option<String>,
    task: Option<Task>,
}

#[derive(Serialize)]
pub struct UpdateTaskResponse {
    error: Option<String>,
    task: Task,
}

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

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
        Ok(tasks) => warp::reply::with_status(
            warp::reply::json(&GetTaskResponse {
                error: None,
                tasks: Some(tasks),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetTaskResponse {
                error: Some("Failed to query for tasks".to_string()),
                tasks: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

fn run_create_task(
    token: String,
    description: String,
    pool: &PgPool,
) -> Result<Task, TaskQueryError> {
    Ok(Task::create(&get_conn(&pool).unwrap(), token, description)?)
}

pub async fn create_task(
    new_task: ApiNewTask,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "create_task: token={}, new_task={:?}",
        session_token, new_task
    );
    Ok(
        match run_create_task(session_token, new_task.description, &db_pool) {
            Ok(task) => warp::reply::with_status(
                warp::reply::json(&CreateTaskResponse {
                    error: None,
                    task: Some(task),
                }),
                StatusCode::OK,
            ),
            Err(_) => warp::reply::with_status(
                warp::reply::json(&CreateTaskResponse {
                    error: Some("Failed to create task".to_string()),
                    task: None,
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        },
    )
}

pub async fn update_task(
    id: u64,
    task_update: ApiNewTask,
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
