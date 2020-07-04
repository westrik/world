use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

type Identifier = String;

#[derive(Debug)]
// Reference: https://tools.ietf.org/html/rfc7231#section-6.5.1
pub enum ApiError {
    // HTTP 400
    InvalidRequest(String),
    // HTTP 403
    Forbidden,
    // HTTP 404
    NotFound(Identifier),
    // TODO: 413 Payload Too Large
    // TODO: 415 Unsupported Media Type (for uploads)
    // TODO: 426 Upgrade Required (for HTTP/2 or /3)
    // TODO: 429 Too Many Requests
    // HTTP 500
    InternalError(String),
    // HTTP 500 or 404
    DatabaseError(diesel::result::Error),
    // TODO: 503 Service Unavailable (RetryableError(String))
    //   - with Retry-After header field
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
    } else if let Some(ApiError::InvalidRequest(msg)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = format!("Bad request: {}", msg);
    } else if let Some(ApiError::NotFound(identifier)) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = format!("Not found: {}", identifier);
    } else if let Some(ApiError::InternalError(msg)) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = format!("Internal server error: {}", msg);
    } else if let Some(ApiError::Forbidden) = err.find() {
        code = StatusCode::FORBIDDEN;
        message = "Invalid credentials".to_string();
    } else if let Some(ApiError::DatabaseError(db_err)) = err.find() {
        match db_err {
            diesel::result::Error::NotFound => {
                code = StatusCode::NOT_FOUND;
                message = "Not found".to_string();
            },
            _ => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                // TODO: LOG ERROR HERE
                message = "Internal server error".to_string();
            }
            // Error::InvalidCString(_) => {},
            // Error::DatabaseError(_, _) => {},
            // Error::QueryBuilderError(_) => {},
            // Error::DeserializationError(_) => {},
            // Error::SerializationError(_) => {},
            // Error::RollbackTransaction => {},
            // Error::AlreadyInTransaction => {},
            // Error::__Nonexhaustive => {},
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "HTTP method not allowed".to_string();
    } else if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid query string".to_string();
    } else if let Some(_) = err.find::<warp::reject::InvalidHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid header".to_string();
    } else if let Some(_) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = "Missing header".to_string();
    } else if let Some(_) = err.find::<warp::reject::LengthRequired>() {
        code = StatusCode::BAD_REQUEST;
        message = "A content-length header is required".to_string();
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        code = StatusCode::BAD_REQUEST;
        message = "Request payload is too large".to_string();
    } else if let Some(_) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = StatusCode::BAD_REQUEST;
        message = "Request content-type is not supported".to_string();
    } else {
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error".to_string();
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&GenericError {
            error_message: message.into(),
        }),
        code,
    ))
}
