use crate::db::PgPool;
use crate::models::item::ListOptions;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_tasks(
    _opts: ListOptions,
    _session_token: String,
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

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub content: String,
}

pub async fn create_task(
    new_task: NewTask,
    _session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("create_task: {:?}", new_task);
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
    task_update: NewTask,
    _session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("update_task: id={}, task={:?}", id, task_update);
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

pub async fn delete_task(
    id: u64,
    _session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
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

/*
ACTIX IMPLEMENTATIONS:
*/

//#[derive(Serialize)]
//pub struct GetItemResponse {
//    error: Option<String>,
//    items: Option<Vec<Item>>,
//}
//
//fn run_get_items(token: String, pool: &db::PgPool) -> Result<Vec<Item>, ItemQueryError> {
//    Ok(Item::find_all_for_user(&get_conn(&pool).unwrap(), token)?)
//}

// pub async fn get_items(
//     req: HttpRequest,
//     pool: web::Data<db::PgPool>,
// ) -> Result<HttpResponse, Error> {
//     if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
//         let token = String::from(
//             auth_header
//                 .clone()
//                 .to_str()
//                 .map_err(|_| HttpResponse::BadRequest().body("bad token"))?,
//         );
//         let items: Vec<Item> = web::block(move || run_get_items(token, &pool))
//             .await
//             .map_err(|_| HttpResponse::BadRequest().body("failed to find items"))?;
//         Ok(HttpResponse::Ok().json(GetItemResponse {
//             error: None,
//             items: Some(items),
//         }))
//     } else {
//         Ok(HttpResponse::BadRequest().body("no token"))
//     }
// }

//#[derive(Deserialize)]
//pub struct NewItem {
//    content: String,
//}
//
//#[derive(Serialize)]
//pub struct CreateItemResponse {
//    error: Option<String>,
//    item: Option<Item>,
//}
//
//fn run_create_item(
//    token: String,
//    content: String,
//    pool: &db::PgPool,
//) -> Result<Item, ItemQueryError> {
//    Ok(Item::create(&get_conn(&pool).unwrap(), token, content)?)
//}
//
// pub async fn create_item(
//     req: HttpRequest,
//     item: Json<NewItem>,
//     pool: web::Data<db::PgPool>,
// ) -> Result<HttpResponse, Error> {
//     let content = String::from(&item.content);
//     if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
//         let token = String::from(
//             auth_header
//                 .clone()
//                 .to_str()
//                 .map_err(|_| HttpResponse::BadRequest().body("bad token"))?,
//         );
//         let item: Item = web::block(move || run_create_item(token, content, &pool))
//             .await
//             .map_err(|_| HttpResponse::BadRequest().body("failed to create item"))?;
//         Ok(HttpResponse::Ok().json(CreateItemResponse {
//             error: None,
//             item: Some(item),
//         }))
//     } else {
//         Ok(HttpResponse::BadRequest().body("no token"))
//     }
// }
