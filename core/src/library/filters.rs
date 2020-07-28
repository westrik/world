use crate::db::DbPool;
use crate::library::handlers;
use crate::library::handlers::{
    ApiLibraryItemBulkCreateSpec,
    ApiLibraryItemUpdateSpec,
    // ApiLibraryItemVersionCreateSpec,
};
use crate::routes::{json_body, with_db, with_session};
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    library_items_list(db_pool.clone())
        .or(library_item_get(db_pool.clone()))
        .or(library_item_create(db_pool.clone()))
        .or(library_item_update(db_pool.clone()))
        .or(library_item_delete(db_pool))
}

/// GET /library-item?offset=3&limit=5
pub fn library_items_list(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("library-item")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::list_library_items)
}

/// GET /library-item/:id
pub fn library_item_get(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("library-item" / String)
        .and(warp::get())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::get_library_item)
}

/// POST /library-item:bulk-create with JSON body
pub fn library_item_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("library-item:bulk-create")
        .and(warp::post())
        .and(json_body::<ApiLibraryItemBulkCreateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::bulk_create_library_items)
}

/// PATCH /library-item/:id with JSON body
pub fn library_item_update(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("library-item" / String)
        .and(warp::patch())
        .and(json_body::<ApiLibraryItemUpdateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::update_library_item)
}

/// DELETE /library-item/:id
pub fn library_item_delete(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("library-item" / String)
        .and(warp::delete())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::delete_library_item)
}

// /// POST /library-item-version with JSON body
// pub fn library_item_version_create(
//     db_pool: DbPool,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("library-item-version")
//         .and(warp::post())
//         .and(json_body::<ApiLibraryItemVersionCreateSpec>())
//         .and(with_session(db_pool.clone()))
//         .and(with_db(db_pool))
//         .and_then(handlers::create_library_item_version)
// }
