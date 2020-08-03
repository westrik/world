use diesel::{sql_types::Text, Expression};
use std::fmt;

use crate::utils::string_transforms::ToIdentifier;

#[derive(Debug, FromSqlRow, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Active,
    Error,
    Done,
}

impl fmt::Display for JobStatus {
    // TODO: write macro to add Debug and this impl? or improve in some other way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ident())
    }
}

impl Expression for JobStatus {
    type SqlType = Text;
}
