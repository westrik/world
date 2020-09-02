use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::external_services::aws::s3::put_object_request::generate_presigned_upload_url;
use crate::library::models::file::FileType;
use crate::library::models::library_item::{
    LibraryItem, LibraryItemCreateSpec, LibraryItemSummary,
};
use crate::library::models::library_item_version::LibraryItemVersion;
use crate::library::models::library_item_version_type::LibraryItemVersionType;
use crate::resource_identifier::{
    generate_resource_identifier, split_resource_identifier, ResourceType,
};
use crate::utils::api_task::run_api_task;
use crate::utils::config::{CONTENT_BUCKET_NAME, MEDIA_DOMAIN_NAME};
use crate::utils::list_options::ListOptions;
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};

#[derive(Serialize)]
pub struct GetLibraryItemsResponse {
    error: Option<String>,
    #[serde(rename = "libraryItems")]
    library_items: Option<Vec<LibraryItem>>,
}

fn run_list_library_items(
    session: Session,
    pool: &DbPool,
    options: ListOptions,
) -> Result<Vec<LibraryItem>, ApiError> {
    Ok(LibraryItem::list(
        &get_conn(&pool).unwrap(),
        session.user_id,
        options,
    )?)
}

pub async fn list_library_items(
    session: Session,
    db_pool: DbPool,
    options: ListOptions,
) -> Result<impl warp::Reply, Rejection> {
    // TODO: include links to previews in response
    debug!("list_library_items: opts={:?}", &options);
    let library_items =
        run_api_task(move || run_list_library_items(session, &db_pool, options)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetLibraryItemsResponse {
            error: None,
            library_items: Some(library_items),
        }),
        StatusCode::OK,
    ))
}

#[derive(Serialize)]
pub struct GetLibraryItemResponse {
    error: Option<String>,
    #[serde(rename = "libraryItem")]
    library_item: Option<LibraryItem>,
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
    // TODO: include link to preview + full asset in response
    debug!("get_library_item: api_id={:?}", api_id);
    let library_item =
        run_api_task(move || run_get_library_item(session, &db_pool, api_id)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetLibraryItemResponse {
            error: None,
            library_item: Some(library_item),
        }),
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemBulkCreateSpec {
    #[serde(rename = "fileSpecs")]
    pub file_specs: Vec<(i64, FileType)>,
}
#[derive(Serialize)]
pub struct BulkCreateLibraryItemsResponse {
    error: Option<String>,
    #[serde(rename = "libraryItems")]
    library_items: Option<Vec<LibraryItemSummary>>,
}

async fn run_bulk_create_library_items(
    spec: ApiLibraryItemBulkCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Vec<LibraryItemSummary>, ApiError> {
    let conn = get_conn(db_pool).unwrap();
    let user = run_api_task(move || session.get_user(&conn)).await?;
    // TODO: limit number of files per request
    // TODO: limit maximum file size?
    let create_specs = {
        let mut specs = vec![];
        for (file_size, file_type) in spec.file_specs.iter() {
            let name = generate_mnemonic(DEFAULT_MNEMONIC_LENGTH);
            let api_id = generate_resource_identifier(ResourceType::LibraryItem);
            let file_name = format!(
                "{}/{}-{}.{}",
                split_resource_identifier(&user.api_id),
                split_resource_identifier(&api_id),
                name,
                file_type
            );
            let presigned_upload_url = generate_presigned_upload_url(
                (*CONTENT_BUCKET_NAME).to_string(),
                file_name.to_string(),
                *file_size,
            )
            .await?;
            specs.push(LibraryItemCreateSpec {
                api_id,
                user_id: user.id,
                name: name.to_string(),
                presigned_upload_url: Some(presigned_upload_url),
                uploaded_file_name: Some(file_name.to_string()),
                uploaded_file_size_bytes: Some(*file_size),
            });
        }
        specs
    };

    let conn = get_conn(db_pool).unwrap();
    Ok(run_api_task(move || LibraryItem::bulk_create(&conn, create_specs)).await?)
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

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemUpdateSpec {
    pub name: Option<String>,
}
#[derive(Serialize)]
pub struct UpdateLibraryItemResponse {
    error: Option<String>,
    #[serde(rename = "libraryItem")]
    library_item: Option<LibraryItemSummary>,
    #[serde(rename = "uploadUrl")]
    upload_url: Option<String>,
}

fn run_update_library_item(
    session: Session,
    api_id: String,
    spec: ApiLibraryItemUpdateSpec,
    pool: &DbPool,
) -> Result<LibraryItemSummary, ApiError> {
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
    let library_item =
        run_api_task(move || run_update_library_item(session, api_id, spec, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateLibraryItemResponse {
            error: None,
            library_item: Some(library_item),
            upload_url: None,
        }),
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiLibraryItemVersionCreateSpec {
    #[serde(rename = "libraryItemId")]
    library_item_api_id: String,
}
#[derive(Serialize)]
pub struct CreateLibraryItemVersionResponse {
    error: Option<String>,
    #[serde(rename = "libraryItemVersion")]
    library_item_version: Option<LibraryItemVersion>,
}

fn run_create_library_item_version(
    spec: ApiLibraryItemVersionCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<LibraryItemVersion, ApiError> {
    let conn = &get_conn(&db_pool).unwrap();
    let library_item = LibraryItemSummary::find(conn, session.user_id, spec.library_item_api_id)?;
    let file_name = match &library_item.uploaded_file_name {
        Some(name) => Ok(name),
        None => Err(ApiError::InvalidRequest(
            "Can't create library item version for library item with no upload attempts"
                .to_string(),
        )),
    }?;
    let library_item_version_url = format!("https://{}/{}", *MEDIA_DOMAIN_NAME, &file_name);
    let library_item_version = LibraryItemVersion::create(
        conn,
        session.user_id,
        library_item,
        LibraryItemVersionType::Original,
        Some(library_item_version_url),
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
    let library_item_version =
        run_api_task(move || run_create_library_item_version(spec, session, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&CreateLibraryItemVersionResponse {
            error: None,
            library_item_version: Some(library_item_version),
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
