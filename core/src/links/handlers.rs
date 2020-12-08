use serde_json::json;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::external_services::aws::s3::put_object::generate_presigned_upload_url;
use crate::jobs::enqueue_job::enqueue_job;
use crate::jobs::job_type::JobType;
use crate::media::models::file::FileType;
use crate::media::models::media_item::{MediaItem, MediaItemCreateSpec, MediaItemSummary};
use crate::media::models::media_item_version::MediaItemVersion;
use crate::media::models::media_item_version_type::MediaItemVersionType;
use crate::resource_identifier::{
    generate_resource_identifier, split_resource_identifier, ResourceType,
};
use crate::utils::api_task::run_api_task;
use crate::utils::config::{CONTENT_BUCKET_NAME, MEDIA_DOMAIN_NAME};
use crate::utils::list_options::ListOptions;
use crate::utils::mnemonic::{generate_mnemonic, DEFAULT_MNEMONIC_LENGTH};

#[derive(Serialize)]
pub struct GetMediaItemsResponse {
    error: Option<String>,
    #[serde(rename = "mediaItems")]
    media_items: Option<Vec<MediaItem>>,
}

fn run_list_media_items(
    session: Session,
    pool: &DbPool,
    options: ListOptions,
) -> Result<Vec<MediaItem>, ApiError> {
    Ok(MediaItem::list(
        &get_conn(&pool).unwrap(),
        session.user_id,
        options,
    )?)
}

pub async fn list_media_items(
    session: Session,
    db_pool: DbPool,
    options: ListOptions,
) -> Result<impl warp::Reply, Rejection> {
    // TODO: include links to previews in response
    debug!("list_media_items: opts={:?}", &options);
    let media_items =
        run_api_task(move || run_list_media_items(session, &db_pool, options)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetMediaItemsResponse {
            error: None,
            media_items: Some(media_items),
        }),
        StatusCode::OK,
    ))
}

#[derive(Serialize)]
pub struct GetMediaItemResponse {
    error: Option<String>,
    #[serde(rename = "mediaItem")]
    media_item: Option<MediaItem>,
}

fn run_get_media_item(
    session: Session,
    pool: &DbPool,
    api_id: String,
) -> Result<MediaItem, ApiError> {
    Ok(MediaItem::find(
        &get_conn(&pool).unwrap(),
        session.user_id,
        api_id,
    )?)
}

pub async fn get_media_item(
    api_id: String,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    // TODO: include link to preview + full asset in response
    debug!("get_media_item: api_id={:?}", api_id);
    let media_item = run_api_task(move || run_get_media_item(session, &db_pool, api_id)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetMediaItemResponse {
            error: None,
            media_item: Some(media_item),
        }),
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiMediaItemBulkCreateSpec {
    #[serde(rename = "fileSpecs")]
    pub file_specs: Vec<(i64, FileType)>,
}
#[derive(Serialize)]
pub struct BulkCreateMediaItemsResponse {
    error: Option<String>,
    #[serde(rename = "mediaItems")]
    media_items: Option<Vec<MediaItemSummary>>,
}

async fn run_bulk_create_media_items(
    spec: ApiMediaItemBulkCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Vec<MediaItemSummary>, ApiError> {
    let conn = get_conn(db_pool).unwrap();
    let user = run_api_task(move || session.get_user(&conn)).await?;
    // TODO: limit number of files per request
    // TODO: limit maximum file size?
    let create_specs = {
        let mut specs = vec![];
        for (file_size, file_type) in spec.file_specs.iter() {
            let name = generate_mnemonic(DEFAULT_MNEMONIC_LENGTH);
            let api_id = generate_resource_identifier(ResourceType::MediaItem);
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
            specs.push(MediaItemCreateSpec {
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
    Ok(run_api_task(move || MediaItem::bulk_create(&conn, create_specs)).await?)
}

pub async fn bulk_create_media_items(
    spec: ApiMediaItemBulkCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("bulk_create_media_items: spec={:?}", spec);
    let media_items = Some(run_bulk_create_media_items(spec, session, &db_pool).await?);
    Ok(warp::reply::with_status(
        warp::reply::json(&BulkCreateMediaItemsResponse {
            error: None,
            media_items,
        }),
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiMediaItemUpdateSpec {
    pub name: Option<String>,
}
#[derive(Serialize)]
pub struct UpdateMediaItemResponse {
    error: Option<String>,
    #[serde(rename = "mediaItem")]
    media_item: Option<MediaItemSummary>,
    #[serde(rename = "uploadUrl")]
    upload_url: Option<String>,
}

fn run_update_media_item(
    session: Session,
    api_id: String,
    spec: ApiMediaItemUpdateSpec,
    pool: &DbPool,
) -> Result<MediaItemSummary, ApiError> {
    Ok(MediaItem::update(
        &get_conn(&pool).unwrap(),
        session.user_id,
        api_id,
        spec.name,
    )?)
}

pub async fn update_media_item(
    api_id: String,
    spec: ApiMediaItemUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("update_media_item: api_id={}, spec={:?}", api_id, spec);
    let media_item =
        run_api_task(move || run_update_media_item(session, api_id, spec, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateMediaItemResponse {
            error: None,
            media_item: Some(media_item),
            upload_url: None,
        }),
        StatusCode::OK,
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiMediaItemVersionCreateSpec {
    #[serde(rename = "mediaItemId")]
    media_item_api_id: String,
}
#[derive(Serialize)]
pub struct CreateMediaItemVersionResponse {
    error: Option<String>,
    #[serde(rename = "mediaItemVersion")]
    media_item_version: Option<MediaItemVersion>,
}

fn run_create_media_item_version(
    spec: ApiMediaItemVersionCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<MediaItemVersion, ApiError> {
    let conn = &get_conn(&db_pool).unwrap();
    let media_item = MediaItemSummary::find(conn, session.user_id, spec.media_item_api_id)?;
    let file_name = match &media_item.uploaded_file_name {
        Some(name) => Ok(name.to_string()),
        None => Err(ApiError::InvalidRequest(
            "Can't create media item version for media item with no upload attempts".to_string(),
        )),
    }?;
    let media_item_version_url = format!("https://{}/{}", *MEDIA_DOMAIN_NAME, &file_name);
    let media_item_version = MediaItemVersion::create(
        conn,
        session.user_id,
        media_item,
        MediaItemVersionType::Original,
        Some(media_item_version_url),
    )?;
    if let Err(e) = enqueue_job(
        &conn,
        Some(session.user_id),
        JobType::IngestMediaUpload,
        // TODO: payload should be part of enum
        Some(json!({
            "file_name": &file_name,
            "media_version_api_id": &media_item_version.api_id,
        })),
    ) {
        error!(
            "failed to enqueue {} job [user_id={}][error={:#?}]",
            JobType::IngestMediaUpload,
            session.user_id,
            e
        );
    }

    Ok(media_item_version)
}

pub async fn create_media_item_version(
    spec: ApiMediaItemVersionCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_media_item_version: spec={:?}", spec);
    let media_item_version =
        run_api_task(move || run_create_media_item_version(spec, session, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&CreateMediaItemVersionResponse {
            error: None,
            media_item_version: Some(media_item_version),
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_media_item(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_media_item: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}
