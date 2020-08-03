use crate::jobs::errors::JobError;
use diesel::{sql_types::Text, Expression};
use std::fmt;
use std::str::FromStr;

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

impl FromStr for JobType {
    type Err = JobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(JobType::System),
            // TODO: convert to snake-case
            "sendemail" => Ok(JobType::SendEmail),
            _ => Err(JobError::InvalidJob(format!("Invalid job type: {}", s))),
        }
    }
}
