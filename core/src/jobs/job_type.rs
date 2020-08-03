use diesel::{sql_types::Text, Expression};
use std::fmt;
use std::str::FromStr;

use crate::jobs::errors::JobError;
use crate::utils::string_transforms::ToIdentifier;

#[derive(Debug, Serialize, Deserialize)]
pub enum JobType {
    DummyJob,
    SendEmail,
}

impl fmt::Display for JobType {
    // TODO: write macro to add Debug and this impl? or improve in some other way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ident())
    }
}

impl Expression for JobType {
    type SqlType = Text;
}

impl FromStr for JobType {
    type Err = JobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dummy_job" => Ok(JobType::DummyJob),
            "send_email" => Ok(JobType::SendEmail),
            _ => Err(JobError::InvalidJob(format!("invalid job type: {}", s))),
        }
    }
}
