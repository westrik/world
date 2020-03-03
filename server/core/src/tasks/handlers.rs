use crate::db::{get_conn, PgPool};
use crate::tasks::models::task::{
    ApiTask, ApiTaskCreateSpec, ApiTaskUpdateSpec, Task, TaskQueryError,
};
use crate::utils::list_options::ListOptions;
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
pub struct GetTaskResponse {
    error: Option<String>,
    tasks: Option<Vec<ApiTask>>,
}

#[derive(Serialize)]
pub struct UpdateTaskResponse {
    error: Option<String>,
    task: Option<ApiTask>,
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
    debug!("list_tasks: opts={:?}", opts);
    Ok(match run_get_tasks(session_token, &db_pool) {
        Ok(tasks) => warp::reply::with_status(
            warp::reply::json(&GetTaskResponse {
                error: None,
                tasks: Some(tasks.iter().map(ApiTask::from).collect()),
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
    new_task: ApiTaskCreateSpec,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("create_task: new_task={:?}", new_task);
    Ok(
        match run_create_task(session_token, new_task.description, &db_pool) {
            Ok(task) => warp::reply::with_status(
                warp::reply::json(&UpdateTaskResponse {
                    error: None,
                    task: Some(ApiTask::from(&task)),
                }),
                StatusCode::OK,
            ),
            Err(_) => warp::reply::with_status(
                warp::reply::json(&UpdateTaskResponse {
                    error: Some("Failed to create task".to_string()),
                    task: None,
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        },
    )
}

fn run_update_task(
    token: String,
    api_id: String,
    spec: ApiTaskUpdateSpec,
    pool: &PgPool,
) -> Result<Task, TaskQueryError> {
    Ok(Task::update(
        &get_conn(&pool).unwrap(),
        token,
        api_id,
        spec,
    )?)
}

pub async fn update_task(
    api_id: String,
    spec: ApiTaskUpdateSpec,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("update_task: api_id={}, spec={:?}", api_id, spec);
    Ok(
        match run_update_task(session_token, api_id, spec, &db_pool) {
            Ok(task) => warp::reply::with_status(
                warp::reply::json(&UpdateTaskResponse {
                    error: None,
                    task: Some(ApiTask::from(&task)),
                }),
                StatusCode::OK,
            ),
            Err(_) => warp::reply::with_status(
                warp::reply::json(&UpdateTaskResponse {
                    error: Some("Failed to create task".to_string()),
                    task: None,
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        },
    )
}

pub async fn delete_task(
    api_id: String,
    _session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_task: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}