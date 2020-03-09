#[derive(Debug, PartialEq)]
pub enum TaskError {
    TaskNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}
