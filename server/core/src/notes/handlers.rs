use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::notes::content::parsing::parse_markdown_content;
use crate::notes::content::schema::Content;
use crate::notes::models::note::{Note, NoteSummary};
use crate::utils::list_options::ListOptions;
use std::convert::Infallible;
use warp::http::StatusCode;

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
    pub content: Option<Content>,
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

fn run_get_notes(session: Session, pool: &DbPool) -> Result<Vec<NoteSummary>, ApiError> {
    Ok(Note::find_all(&get_conn(&pool).unwrap(), session)?)
}

pub async fn list_notes(
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("list_notes: opts={:?}", opts);
    Ok(match run_get_notes(session, &db_pool) {
        Ok(notes) => warp::reply::with_status(
            warp::reply::json(&GetNotesResponse {
                error: None,
                notes: Some(notes),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetNotesResponse {
                error: Some("Failed to query for notes".to_string()),
                notes: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

fn run_get_note(session: Session, pool: &DbPool, api_id: String) -> Result<Note, ApiError> {
    Ok(Note::find(&get_conn(&pool).unwrap(), session, api_id)?)
}

pub async fn get_note(
    api_id: String,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("get_note: api_id={:?}", api_id);
    Ok(match run_get_note(session, &db_pool, api_id) {
        Ok(note) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: None,
                note: Some(note),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: Some("Failed to load note".to_string()),
                note: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

fn run_create_note(
    spec: ApiNoteCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Note, ApiError> {
    let content_json: serde_json::Value;
    if let Some(content) = spec.content_json {
        content_json = content;
    } else if let Some(content) = spec.content_raw {
        content_json = serde_json::to_value(&parse_markdown_content(content))
            .map_err(|_| ApiError::InternalError("Bad content conversion".to_string()))?;
    } else {
        return Err(ApiError::InvalidRequest("No specified content".to_string()));
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
) -> Result<impl warp::Reply, Infallible> {
    debug!("create_note: spec={:?}", spec);
    Ok(match run_create_note(spec, session, &db_pool) {
        Ok(note) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: None,
                note: Some(note),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: Some("Failed to create note".to_string()),
                note: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

fn run_update_note(
    session: Session,
    api_id: String,
    spec: ApiNoteUpdateSpec,
    pool: &DbPool,
) -> Result<Note, ApiError> {
    Ok(Note::update(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
        spec.content,
    )?)
}

pub async fn update_note(
    api_id: String,
    spec: ApiNoteUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("update_note: api_id={}, spec={:?}", api_id, spec);
    Ok(match run_update_note(session, api_id, spec, &db_pool) {
        Ok(note) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: None,
                note: Some(note),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: Some("Failed to update note".to_string()),
                note: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

pub async fn delete_note(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_note: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}
