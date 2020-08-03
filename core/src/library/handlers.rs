use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::library::models::library_item::{LibraryItem, LibraryItemCreateSpec};
// use crate::library::models::library_item_version::LibraryItemVersion;
use crate::library::models::library_item_version::LibraryItemVersion;
use crate::library::models::library_item_version_type::LibraryItemVersionType;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::s3::put_object_request::generate_presigned_upload_url;
use crate::utils::list_options::ListOptions;
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};
use std::convert::Infallible;
use std::env;
use warp::http::StatusCode;
use warp::Rejection;

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemBulkCreateSpec {
    #[serde(rename = "fileSizesInBytes")]
    pub file_sizes_in_bytes: Vec<i64>,
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
pub struct BulkCreateLibraryItemsResponse {
    error: Option<String>,
    #[serde(rename = "libraryItems")]
    library_items: Option<Vec<LibraryItem>>,
}

#[derive(Serialize)]
pub struct UpdateLibraryItemResponse {
    error: Option<String>,
    #[serde(rename = "libraryItem")]
    library_item: Option<LibraryItem>,
    #[serde(rename = "uploadUrl")]
    upload_url: Option<String>,
}

#[derive(Serialize)]
pub struct CreateLibraryItemVersionResponse {
    error: Option<String>,
    #[serde(rename = "libraryItemVersion")]
    library_item_version: Option<LibraryItemVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemVersionCreateSpec {
    #[serde(rename = "libraryItemId")]
    library_item_api_id: String,
}

fn run_get_library_items(session: Session, pool: &DbPool) -> Result<Vec<LibraryItem>, ApiError> {
    Ok(LibraryItem::find_all(
        &get_conn(&pool).unwrap(),
        session.user_id,
    )?)
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
        session.user_id,
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

async fn run_bulk_create_library_items(
    spec: ApiLibraryItemBulkCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Vec<LibraryItem>, ApiError> {
    // TODO: limit number of files per request
    // TODO: limit maximum file size?
    lazy_static! {
        static ref CONTENT_BUCKET_NAME: String =
            env::var("CONTENT_BUCKET_NAME").expect("CONTENT_BUCKET_NAME must be set");
    }
    let create_specs = {
        let mut specs = vec![];
        for file_size in spec.file_sizes_in_bytes.iter() {
            let name = generate_mnemonic(DEFAULT_MNEMONIC_LENGTH);
            let presigned_upload_url = generate_presigned_upload_url(
                (*CONTENT_BUCKET_NAME).to_string(),
                name.to_string(),
                *file_size,
            )
            .await?;
            specs.push(LibraryItemCreateSpec {
                api_id: generate_resource_identifier(ResourceType::LibraryItem),
                user_id: session.user_id,
                name: name.to_string(),
                presigned_upload_url: Some(presigned_upload_url),
                uploaded_file_name: None,
                uploaded_file_size_bytes: Some(*file_size),
            });
        }
        specs
    };

    Ok(LibraryItem::bulk_create(
        &get_conn(&db_pool).unwrap(),
        create_specs,
    )?)
}

pub async fn bulk_create_library_items(
    spec: ApiLibraryItemBulkCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("bulk_create_library_items: spec={:?}", spec);
    let library_items = Some(run_bulk_create_library_items(spec, session, &db_pool).await?);
    Ok(warp::reply::with_status(
        warp::reply::json(&BulkCreateLibraryItemsResponse {
            error: None,
            library_items,
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
        session.user_id,
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
            upload_url: None,
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

fn run_create_library_item_version(
    spec: ApiLibraryItemVersionCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<LibraryItemVersion, ApiError> {
    let conn = &get_conn(&db_pool).unwrap();
    let library_item = LibraryItem::find(conn, session.user_id, spec.library_item_api_id)?;
    // TODO: generate CloudFront URL for LibraryItemVersion (using library_item.{name,api_id})
    // let library_item_version_url = format!("https://assets.westrik.world/{}-{}", library_item.name, library_item.api_id);
    let library_item_version = LibraryItemVersion::create(
        conn,
        session.user_id,
        library_item,
        LibraryItemVersionType::Original,
        None, // Some(library_item_version_url),
    )?;
    // TODO: enqueue library item processing job for library_item_version.id
    //  - should load library_item and library_item_version, then get file metadata from S3
    //  - call Lambda API endpoints to run relevant processing jobs
    Ok(library_item_version)
}

pub async fn create_library_item_version(
    spec: ApiLibraryItemVersionCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_library_item_version: spec={:?}", spec);
    let library_item_version = run_create_library_item_version(spec, session, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&CreateLibraryItemVersionResponse {
            error: None,
            library_item_version: Some(library_item_version),
        }),
        StatusCode::OK,
    ))
}
