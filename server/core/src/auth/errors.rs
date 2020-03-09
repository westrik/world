#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    DatabaseError(diesel::result::Error),
}
