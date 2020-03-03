use crate::db::PgPool;
use crate::notes::handlers;
use crate::notes::handlers::{ApiNoteCreateSpec, ApiNoteUpdateSpec};
use crate::routes::{json_body, with_db, with_session_token};
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    notes_list(db_pool.clone())
        .or(note_create(db_pool.clone()))
        .or(note_update(db_pool.clone()))
        .or(note_delete(db_pool))
}

/// GET /note?offset=3&limit=5
pub fn notes_list(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::list_notes)
}

/// POST /note with JSON body
pub fn note_create(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note")
        .and(warp::post())
        .and(json_body::<ApiNoteCreateSpec>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::create_note)
}

/// PUT /note/:id with JSON body
pub fn note_update(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note" / String)
        .and(warp::put())
        .and(json_body::<ApiNoteUpdateSpec>())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::update_note)
}

/// DELETE /note/:id
pub fn note_delete(
    db_pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("note" / String)
        .and(warp::delete())
        .and(with_session_token())
        .and(with_db(db_pool))
        .and_then(handlers::delete_note)
}
