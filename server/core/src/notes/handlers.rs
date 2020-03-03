use crate::db::{get_conn, PgPool};
use crate::notes::models::note::{Note, NoteQueryError};
use crate::utils::list_options::ListOptions;
use chrono::{DateTime, Utc};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ApiNote {
    pub apiId: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub content: serde_json::Value,
}
#[derive(Debug, Deserialize)]
pub struct ApiNoteCreateSpec {
    pub content: serde_json::Value,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiNoteUpdateSpec {
    pub content: serde_json::Value,
    pub updatedAt: DateTime<Utc>,
}
impl From<&Note> for ApiNote {
    fn from(note: &Note) -> Self {
        ApiNote {
            apiId: note.api_id.clone(),
            content: note.content.clone(),
            createdAt: note.created_at,
            updatedAt: note.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct GetNoteResponse {
    error: Option<String>,
    notes: Option<Vec<ApiNote>>,
}

#[derive(Serialize)]
pub struct UpdateNoteResponse {
    error: Option<String>,
    note: Option<ApiNote>,
}

// TODO: wrap DB queries in blocking task (https://tokio.rs/docs/going-deeper/tasks/)

fn run_get_notes(token: String, pool: &PgPool) -> Result<Vec<Note>, NoteQueryError> {
    Ok(Note::find_all_for_user(&get_conn(&pool).unwrap(), token)?)
}

pub async fn list_notes(
    opts: ListOptions,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("list_notes: opts={:?}", opts);
    Ok(match run_get_notes(session_token, &db_pool) {
        Ok(notes) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: None,
                notes: Some(notes.iter().map(ApiNote::from).collect()),
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&GetNoteResponse {
                error: Some("Failed to query for notes".to_string()),
                notes: None,
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}

fn run_create_note(
    token: String,
    content: serde_json::Value,
    pool: &PgPool,
) -> Result<Note, NoteQueryError> {
    Ok(Note::create(&get_conn(&pool).unwrap(), token, content)?)
}

pub async fn create_note(
    spec: ApiNoteCreateSpec,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("create_note: spec={:?}", spec);
    Ok(
        match run_create_note(session_token, spec.content, &db_pool) {
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
        },
    )
}

fn run_update_note(
    token: String,
    api_id: String,
    spec: ApiNoteUpdateSpec,
    pool: &PgPool,
) -> Result<Note, NoteQueryError> {
    Ok(Note::update(
        &get_conn(&pool).unwrap(),
        token,
        api_id,
        spec,
    )?)
}

pub async fn update_note(
    api_id: String,
    spec: ApiNoteUpdateSpec,
    session_token: String,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("update_note: api_id={}, spec={:?}", api_id, spec);
    Ok(
        match run_update_note(session_token, api_id, spec, &db_pool) {
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
        },
    )
}

pub async fn delete_note(
    api_id: String,
    _session_token: String,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_note: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}