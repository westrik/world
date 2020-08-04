use tera::Tera;
use world_core::jobs::errors::JobError;

pub fn populate_email_template() -> Result<String, JobError> {
    let _templates = match Tera::new("templates/**/*.html") {
        Ok(t) => Ok(t),
        Err(_) => Err(JobError::InternalError(
            "Could not find email templates".to_string(),
        )),
    }?;
    Ok("TEMPLATE".to_string())
}

#[cfg(test)]
pub mod email_template_population {
    use super::*;

    #[test]
    fn basic() {
        let populated_template = populate_email_template();
        assert_eq!(populated_template.unwrap(), "TEMPLATE".to_string());
    }
}
