use std::fmt;

use diesel::sql_types::Text;
use diesel::Expression;

#[derive(Debug, Serialize, Deserialize)]
pub enum JobType {
    SYSTEM,
}

impl fmt::Display for JobType {
    // TODO: write macro to add Debug and this impl
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ascii_lowercase())
    }
}

impl Expression for JobType {
    type SqlType = Text;
}
