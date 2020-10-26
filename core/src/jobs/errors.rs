use crate::errors::ApiError;

type Identifier = String;

#[allow(dead_code)]
#[derive(Debug)]
pub enum JobError {
    InvalidJob(String),
    Forbidden,
    NotFound(Identifier),
    InternalError(String),
    DatabaseError(String),
}

impl From<ApiError> for JobError {
    fn from(api_error: ApiError) -> Self {
        match api_error {
            ApiError::InvalidRequest(msg) => JobError::InvalidJob(msg),
            ApiError::Forbidden => JobError::Forbidden,
            ApiError::NotFound(ident) => JobError::NotFound(ident),
            ApiError::InternalError(msg) => JobError::InternalError(msg),
            ApiError::DatabaseError(err) => JobError::DatabaseError(format!("{:?}", err)),
            ApiError::InternalRuntimeError(msg) => {
                JobError::InternalError(format!("Internal runtime error: {:?}", msg))
            }
        }
    }
}
