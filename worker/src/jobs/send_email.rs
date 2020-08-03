use std::{env, fmt};
use world_core::{jobs::errors::JobError, utils::string_transforms::ToIdentifier};

use crate::jobs::Runnable;

#[derive(Debug, Deserialize)]
pub enum EmailTemplate {
    LoginNotification,
}

impl fmt::Display for EmailTemplate {
    // TODO: write macro to add Debug and this impl? or improve in some other way
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = format!("{:?}", self);
        write!(f, "{}", type_str.to_ident())
    }
}

#[derive(Deserialize)]
pub struct SendEmailJob {
    pub template: EmailTemplate,
    pub recipients: Vec<String>,
}

lazy_static! {
    static ref OUTBOUND_EMAIL_SENDER: String =
        env::var("OUTBOUND_EMAIL_SENDER").expect("OUTBOUND_EMAIL_SENDER must be set");
}

impl Runnable for SendEmailJob {
    fn run(&self) -> Result<String, JobError> {
        // TODO: validate input
        // TODO: populate templates
        // TODO: send request to SendGrid (via external-service-proxy Lambda)
        info!(
            "sending email to {:#?} (template: {})",
            self.recipients, self.template
        );
        Ok("DONE".to_string())
    }
}
