use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

type Identifier = String;

#[derive(Debug)]
pub enum ApiError {
    // our bad (500s)
    DatabaseError(diesel::result::Error),
    InternalError(String),

    // their bad (400s)
    Forbidden,
    NotFound(Identifier),
    InvalidRequest(String),
}

impl warp::reject::Reject for ApiError {}

impl From<ApiError> for warp::reject::Rejection {
    fn from(e: ApiError) -> Self {
        warp::reject::custom(e)
    }
}

#[derive(Debug, Serialize)]
pub struct GenericError {
    #[serde(rename = "errorMessage")]
    error_message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found".to_string();
    } else if let Some(ApiError::NotFound(identifier)) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = format!("Not found: {}", identifier);
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed".to_string();
    } else {
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error".to_string();
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&GenericError {
            error_message: message.into(),
        }),
        code,
    ))
}
