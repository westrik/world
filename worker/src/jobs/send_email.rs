use std::env;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;

#[derive(Deserialize)]
pub struct SendEmailJob {
    template_name: String,
    recipients: Vec<String>,
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
            self.recipients, self.template_name
        );
        Ok("DONE".to_string())
    }
}
