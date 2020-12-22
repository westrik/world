use crate::db::DbPool;
use crate::links::handlers;
use crate::routes::{with_db, with_session};
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_inbound_links_for_note(db_pool)
}

/// GET /link?targetNoteId=note_abcd1234&offset=3&limit=5
pub fn list_inbound_links_for_note(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("link")
        .and(warp::get())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and(warp::query::<ListOptions>())
        .and_then(handlers::list_inbound_links_for_note)
}

/*
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

 */
