use crate::db::PgPool;
use crate::models::item::{Item, ListOptions};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_tasks(
    _opts: ListOptions,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    //    // Just return a JSON array of tasks, applying the limit and offset.
    //    let tasks = db.lock().await;
    //    let tasks: Vec<Todo> = tasks
    //        .clone()
    //        .into_iter()
    //        .skip(opts.offset.unwrap_or(0))
    //        .take(opts.limit.unwrap_or(std::usize::MAX))
    //        .collect();
    //    Ok(warp::reply::json(&tasks))

    Ok(StatusCode::OK)
}

pub async fn create_task(create: Item, _db_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_task: {:?}", create);
    //
    //    let mut vec = db.lock().await;
    //
    //    for task in vec.iter() {
    //        if task.id == create.id {
    //            log::debug!("    -> id already exists: {}", create.id);
    //            // Todo with id already exists, return `400 BadRequest`.
    //            return Ok(StatusCode::BAD_REQUEST);
    //        }
    //    }
    //
    //    // No existing Todo with id, so insert and return `201 Created`.
    //    vec.push(create);

    Ok(StatusCode::CREATED)
}

pub async fn update_task(
    id: u64,
    update: Item,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_task: id={}, task={:?}", id, update);
    return Ok(StatusCode::OK);

    //    let mut vec = db.lock().await;
    //
    //    // Look for the specified Todo...
    //    for task in vec.iter_mut() {
    //        if task.id == id {
    //            *task = update;
    //            return Ok(StatusCode::OK);
    //        }
    //    }
    //
    //    log::debug!("    -> task id not found!");
    //
    //    // If the for loop didn't return OK, then the ID doesn't exist...
    //    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_task(id: u64, _db_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_task: id={}", id);
    Ok(StatusCode::NO_CONTENT)

    //    let mut vec = db.lock().await;
    //
    //    let len = vec.len();
    //    vec.retain(|task| {
    //        // Retain all Todos that aren't this id...
    //        // In other words, remove all that *are* this id...
    //        task.id != id
    //    });
    //
    //    // If the vec is smaller, we found and deleted a Todo!
    //    let deleted = vec.len() != len;
    //
    //    if deleted {
    //        // respond with a `204 No Content`, which means successful,
    //        // yet no body expected...
    //        Ok(StatusCode::NO_CONTENT)
    //    } else {
    //        log::debug!("    -> task id not found!");
    //        Ok(StatusCode::NOT_FOUND)
    //    }
}
