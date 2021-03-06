use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::tasks::models::task::{ApiTask, ApiTaskCreateSpec, ApiTaskUpdateSpec, Task};
use crate::utils::api_task::run_api_task;
use crate::utils::list_options::ListOptions;

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

fn run_get_tasks(session: Session, pool: &DbPool) -> Result<Vec<Task>, ApiError> {
    Ok(Task::find_all_for_user(&get_conn(&pool).unwrap(), session)?)
}

pub async fn list_tasks(
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_tasks: opts={:?}", opts);
    let tasks = run_api_task(move || run_get_tasks(session, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetTaskResponse {
            error: None,
            tasks: Some(tasks.iter().map(ApiTask::from).collect()),
        }),
        StatusCode::OK,
    ))
}

fn run_create_task(session: Session, description: String, pool: &DbPool) -> Result<Task, ApiError> {
    Ok(Task::create(
        &get_conn(&pool).unwrap(),
        session,
        description,
    )?)
}

pub async fn create_task(
    new_task: ApiTaskCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_task: new_task={:?}", new_task);
    let task =
        run_api_task(move || run_create_task(session, new_task.description, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateTaskResponse {
            error: None,
            task: Some(ApiTask::from(&task)),
        }),
        StatusCode::OK,
    ))
}

fn run_update_task(
    session: Session,
    api_id: String,
    spec: ApiTaskUpdateSpec,
    pool: &DbPool,
) -> Result<Task, ApiError> {
    Ok(Task::update(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
        spec,
    )?)
}

pub async fn update_task(
    api_id: String,
    spec: ApiTaskUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("update_task: api_id={}, spec={:?}", api_id, spec);
    let task = run_api_task(move || run_update_task(session, api_id, spec, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateTaskResponse {
            error: None,
            task: Some(ApiTask::from(&task)),
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_task(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_task: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}
