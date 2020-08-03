type Identifier = String;

#[allow(dead_code)]
#[derive(Debug)]
pub enum JobError {
    InvalidJob(String),
    Forbidden(Identifier),
    NotFound(Identifier),
    InternalError(String),
    DatabaseError(diesel::result::Error),
}
