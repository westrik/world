use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::notes::models::note::{Note, NoteSummary};
use crate::notes::parsing::parse_markdown_content;
use crate::utils::list_options::ListOptions;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

#[derive(Debug, Deserialize)]
pub struct ApiNoteCreateSpec {
    #[serde(rename = "contentJson")]
    pub content_json: Option<serde_json::Value>,
    #[serde(rename = "contentRaw")]
    pub content_raw: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct ApiNoteUpdateSpec {
    pub name: Option<String>,
    #[serde(rename = "contentRaw")]
    pub content_raw: Option<String>,
}

#[derive(Serialize)]
pub struct GetNotesResponse {
    error: Option<String>,
    notes: Option<Vec<NoteSummary>>,
}

#[derive(Serialize)]
pub struct GetNoteResponse {
    error: Option<String>,
    note: Option<Note>,
}

#[derive(Serialize)]
pub struct UpdateNoteResponse {
    error: Option<String>,
    note: Option<Note>,
}

fn run_get_notes(session: Session, pool: &DbPool) -> Result<Vec<NoteSummary>, ApiError> {
    Ok(Note::find_all(&get_conn(&pool).unwrap(), session)?)
}

pub async fn list_notes(
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_notes: opts={:?}", opts);
    let notes = run_get_notes(session, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetNotesResponse {
            error: None,
            notes: Some(notes),
        }),
        StatusCode::OK,
    ))
}

fn run_get_note(session: Session, pool: &DbPool, api_id: String) -> Result<Note, ApiError> {
    Ok(Note::find(&get_conn(&pool).unwrap(), session, api_id)?)
}

pub async fn get_note(
    api_id: String,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("get_note: api_id={:?}", api_id);
    let note = run_get_note(session, &db_pool, api_id)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetNoteResponse {
            error: None,
            note: Some(note),
        }),
        StatusCode::OK,
    ))
}

fn run_create_note(
    spec: ApiNoteCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Note, ApiError> {
    let content_json: Option<serde_json::Value>;
    if let Some(content) = spec.content_json {
        content_json = Some(content);
    } else if let Some(content) = spec.content_raw {
        content_json = Some(
            serde_json::to_value(&parse_markdown_content(content))
                .map_err(|_| ApiError::InternalError("Bad content conversion".to_string()))?,
        );
    } else {
        content_json = None;
    }

    Ok(Note::create(
        &get_conn(&db_pool).unwrap(),
        session,
        content_json,
    )?)
}

pub async fn create_note(
    spec: ApiNoteCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_note: spec={:?}", spec);
    let note = run_create_note(spec, session, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateNoteResponse {
            error: None,
            note: Some(note),
        }),
        StatusCode::OK,
    ))
}

fn run_update_note(
    session: Session,
    api_id: String,
    spec: ApiNoteUpdateSpec,
    pool: &DbPool,
) -> Result<Note, ApiError> {
    let content_json: Option<serde_json::Value>;
    if let Some(content_raw) = spec.content_raw {
        content_json = Some(
            serde_json::to_value(&parse_markdown_content(content_raw))
                .map_err(|_| ApiError::InternalError("Bad content conversion".to_string()))?,
        );
    } else {
        content_json = None;
    }

    Ok(Note::update(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
        spec.name,
        content_json,
    )?)
}

pub async fn update_note(
    api_id: String,
    spec: ApiNoteUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("update_note: api_id={}, spec={:?}", api_id, spec);
    let note = run_update_note(session, api_id, spec, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateNoteResponse {
            error: None,
            note: Some(note),
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_note(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_note: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}
