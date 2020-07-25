use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::library::content_upload::put_object_request::generate_presigned_upload_url;
use crate::library::models::library_item::LibraryItem;
use crate::library::models::library_item_version::LibraryItemVersion;
use crate::utils::list_options::ListOptions;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemCreateSpec {
    pub name: Option<String>,
    #[serde(rename = "fileSizesInBytes")]
    pub file_sizes_in_bytes: Option<Vec<i32>>,
}
#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemUpdateSpec {
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct GetLibraryItemsResponse {
    error: Option<String>,
    #[serde(rename = "libraryItems")]
    library_items: Option<Vec<LibraryItem>>,
}

#[derive(Serialize)]
pub struct GetLibraryItemResponse {
    error: Option<String>,
    #[serde(rename = "libraryItem")]
    library_item: Option<LibraryItem>,
}

#[derive(Serialize)]
pub struct UpdateLibraryItemResponse {
    error: Option<String>,
    #[serde(rename = "libraryItem")]
    library_item: Option<LibraryItem>,
    #[serde(rename = "uploadUrlsByFileSize")]
    upload_urls_by_file_size: Option<HashMap<i32, Vec<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemVersionCreateSpec {
    #[serde(rename = "fileSizeInBytes")]
    pub file_size_in_bytes: Option<i32>,
}

fn run_get_library_items(session: Session, pool: &DbPool) -> Result<Vec<LibraryItem>, ApiError> {
    Ok(LibraryItem::find_all(&get_conn(&pool).unwrap(), session)?)
}

pub async fn list_library_items(
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_library_items: opts={:?}", opts);
    let library_items = run_get_library_items(session, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetLibraryItemsResponse {
            error: None,
            library_items: Some(library_items),
        }),
        StatusCode::OK,
    ))
}

fn run_get_library_item(
    session: Session,
    pool: &DbPool,
    api_id: String,
) -> Result<LibraryItem, ApiError> {
    Ok(LibraryItem::find(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
    )?)
}

pub async fn get_library_item(
    api_id: String,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("get_library_item: api_id={:?}", api_id);
    let library_item = run_get_library_item(session, &db_pool, api_id)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetLibraryItemResponse {
            error: None,
            library_item: Some(library_item),
        }),
        StatusCode::OK,
    ))
}

fn run_create_library_item(
    spec: ApiLibraryItemCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<LibraryItem, ApiError> {
    Ok(LibraryItem::create(
        &get_conn(&db_pool).unwrap(),
        session,
        spec.name,
    )?)
}

pub async fn create_library_item(
    spec: ApiLibraryItemCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_library_item: spec={:?}", spec);
    let upload_urls_by_file_size: Option<HashMap<i32, Vec<String>>>;
    // TODO: clean this up
    if let Some(file_sizes_in_bytes) = &spec.file_sizes_in_bytes {
        let mut upload_urls: HashMap<i32, Vec<String>> = HashMap::new();
        for file_size in file_sizes_in_bytes {
            let upload_url = generate_presigned_upload_url(*file_size);
            match upload_urls.get_mut(file_size) {
                Some(urls) => (*urls).push(upload_url),
                None => {
                    let _ = upload_urls.insert(*file_size, vec![upload_url]);
                }
            }
        }
        upload_urls_by_file_size = Some(upload_urls);
    } else {
        upload_urls_by_file_size = None;
    }
    let library_item = run_create_library_item(spec, session, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateLibraryItemResponse {
            error: None,
            library_item: Some(library_item),
            upload_urls_by_file_size,
        }),
        StatusCode::OK,
    ))
}

fn run_update_library_item(
    session: Session,
    api_id: String,
    spec: ApiLibraryItemUpdateSpec,
    pool: &DbPool,
) -> Result<LibraryItem, ApiError> {
    Ok(LibraryItem::update(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
        spec.name,
    )?)
}

pub async fn update_library_item(
    api_id: String,
    spec: ApiLibraryItemUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("update_library_item: api_id={}, spec={:?}", api_id, spec);
    let library_item = run_update_library_item(session, api_id, spec, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateLibraryItemResponse {
            error: None,
            library_item: Some(library_item),
            upload_urls_by_file_size: None,
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_library_item(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_library_item: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}

// fn run_create_library_item_version(
//     spec: ApiLibraryItemVersionCreateSpec,
//     session: Session,
//     db_pool: &DbPool,
// ) -> Result<LibraryItem, ApiError> {
//     Ok(LibraryItemVersion::create(
//         &get_conn(&db_pool).unwrap(),
//         session,
//         spec.name,
//     )?)
// }
//
// pub async fn create_library_item_version(
//     spec: ApiLibraryItemVersionCreateSpec,
//     session: Session,
//     db_pool: DbPool,
// ) -> Result<impl warp::Reply, Rejection> {
//     debug!("create_library_item: spec={:?}", spec);
//     let library_item = run_create_library_item(spec, session, &db_pool)?;
//     Ok(warp::reply::with_status(
//         warp::reply::json(&UpdateLibraryItemResponse {
//             error: None,
//             library_item: Some(library_item),
//         }),
//         StatusCode::OK,
//     ))
// }
