use diesel::{sql_types::Text, Expression};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum JobType {
    System,
    SendEmail,
}

impl fmt::Display for JobType {
    // TODO: write macro to add Debug and this impl? or improve in some other way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        // TODO: convert to snake-case instead
        write!(f, "{}", type_str.to_ascii_lowercase())
    }
}

impl Expression for JobType {
    type SqlType = Text;
}
