use crate::db::{get_conn, PgPool};
use crate::models::item::{Item, ItemQueryError, ListOptions};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
pub struct GetItemResponse {
    error: Option<String>,
    items: Option<Vec<Item>>,
}

fn run_get_items(token: String, pool: &PgPool) -> Result<Vec<Item>, ItemQueryError> {
    Ok(Item::find_all_for_user(&get_conn(&pool).unwrap(), token)?)
}

pub async fn list_tasks(
    opts: ListOptions,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("list_tasks: token={}, opts={:?}", session_token, opts);
    Ok(match run_get_items(session_token, &db_pool) {
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
pub struct CreateItemResponse {
    error: Option<String>,
    item: Option<Item>,
}

fn run_create_item(token: String, content: String, pool: &PgPool) -> Result<Item, ItemQueryError> {
    Ok(Item::create(&get_conn(&pool).unwrap(), token, content)?)
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
        match run_create_item(session_token, new_task.content, &db_pool) {
            Ok(item) => warp::reply::with_status(
                warp::reply::json(&CreateItemResponse {
                    error: None,
                    item: Some(item),
                }),
                StatusCode::OK,
            ),
            Err(_) => warp::reply::with_status(
                warp::reply::json(&CreateItemResponse {
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
