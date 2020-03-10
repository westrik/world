use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::notes::content::parsing::parse_markdown_content;
use crate::notes::content::schema::Content;
use crate::notes::errors::NoteError;
use crate::notes::models::note::Note;
use crate::utils::list_options::ListOptions;
use chrono::{DateTime, Utc};
use std::convert::Infallible;
use warp::http::StatusCode;

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ApiNote {
    pub apiId: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub title: String,
    pub content: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub struct ApiNoteCreateSpec {
    pub content_json: Option<serde_json::Value>,
    pub content_raw: Option<String>,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiNoteUpdateSpec {
    pub content: Option<Content>,
}
impl From<&Note> for ApiNote {
    fn from(note: &Note) -> Self {
        ApiNote {
            apiId: note.api_id.clone(),
            content: note.content.clone(),
            createdAt: note.created_at,
            updatedAt: note.updated_at,
            title: "".to_string(),
        }
    }
}
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ApiNoteSummary {
    pub apiId: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub title: String,
}
impl From<&Note> for ApiNoteSummary {
    fn from(note: &Note) -> Self {
        ApiNoteSummary {
            apiId: note.api_id.clone(),
            createdAt: note.created_at,
            updatedAt: note.updated_at,
            title: "".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct GetNotesResponse {
    error: Option<String>,
    notes: Option<Vec<ApiNoteSummary>>,
}

#[derive(Serialize)]
pub struct UpdateNoteResponse {
    error: Option<String>,
    note: Option<ApiNote>,
}

// TODO: add GET endpoint for API ID param (return full content)

fn run_get_notes(session: Session, pool: &DbPool) -> Result<Vec<Note>, NoteError> {
    // TODO: don't load content when generating listing
    Ok(Note::find_all_for_user(&get_conn(&pool).unwrap(), session)?)
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
                notes: Some(notes.iter().map(ApiNoteSummary::from).collect()),
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

fn run_create_note(
    spec: ApiNoteCreateSpec,
    session: Session,
    db_pool: &DbPool,
) -> Result<Note, NoteError> {
    let content_json: serde_json::Value;
    if let Some(content) = spec.content_json {
        content_json = content;
    } else if let Some(content) = spec.content_raw {
        content_json = serde_json::to_value(&parse_markdown_content(content))
            .map_err(|_| NoteError::BadContentConversion)?;
    } else {
        return Err(NoteError::NoSpecifiedContent);
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
            warp::reply::json(&UpdateNoteResponse {
                error: None,
                note: Some(ApiNote::from(&note)),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&UpdateNoteResponse {
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
) -> Result<Note, NoteError> {
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
            warp::reply::json(&UpdateNoteResponse {
                error: None,
                note: Some(ApiNote::from(&note)),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&UpdateNoteResponse {
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
