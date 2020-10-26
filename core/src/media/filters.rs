use crate::db::DbPool;
use crate::media::handlers;
use crate::media::handlers::{
    ApiMediaItemBulkCreateSpec, ApiMediaItemUpdateSpec, ApiMediaItemVersionCreateSpec,
};
use crate::routes::{json_body, with_db, with_session};
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    media_item_bulk_create(db_pool.clone())
        .or(media_item_delete(db_pool.clone()))
        .or(media_item_get(db_pool.clone()))
        .or(media_item_update(db_pool.clone()))
        .or(media_item_version_create(db_pool.clone()))
        .or(media_items_list(db_pool))
}

/// GET /media-item?offset=3&limit=5
pub fn media_items_list(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item")
        .and(warp::get())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and(warp::query::<ListOptions>())
        .and_then(handlers::list_media_items)
}

/// GET /media-item/:id
pub fn media_item_get(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item" / String)
        .and(warp::get())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::get_media_item)
}

/// POST /media-item:bulk-create with JSON body
pub fn media_item_bulk_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item:bulk-create")
        .and(warp::post())
        .and(json_body::<ApiMediaItemBulkCreateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::bulk_create_media_items)
}

/// PATCH /media-item/:id with JSON body
pub fn media_item_update(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item" / String)
        .and(warp::patch())
        .and(json_body::<ApiMediaItemUpdateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::update_media_item)
}

/// DELETE /media-item/:id
pub fn media_item_delete(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item" / String)
        .and(warp::delete())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::delete_media_item)
}

/// POST /media-item-version with JSON body
pub fn media_item_version_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media-item-version")
        .and(warp::post())
        .and(json_body::<ApiMediaItemVersionCreateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::create_media_item_version)
}
