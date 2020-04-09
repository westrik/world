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
