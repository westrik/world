#[derive(Debug)]
pub enum NoteError {
    // our bad (500s)
    DatabaseError(diesel::result::Error),
    BadContentConversion,

    // their bad (400s)
    NoteNotFound,
    InvalidToken,
    NoSpecifiedContent,
}
