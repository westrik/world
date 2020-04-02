#[derive(Debug)]
pub enum JobError {
    DatabaseError(diesel::result::Error),
    JobNotFound,
}
