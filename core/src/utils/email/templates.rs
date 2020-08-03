use crate::jobs::errors::JobError;
use tera::Tera;

pub fn populate_email_template() -> Result<String, JobError> {
    let templates = match Tera::new("templates/**/*.html") {
        Ok(t) => Ok(t),
        Err(e) => Err(JobError::InternalError(
            "Could not find email templates".to_string(),
        )),
    }?;
}
