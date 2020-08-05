use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use std::env;
use world_core::jobs::errors::JobError;

use crate::emails::templates::populate_email_template;
use crate::jobs::Runnable;

#[derive(Debug, Deserialize)]
pub enum EmailTemplate {
    LoginNotification,
}

#[derive(Deserialize)]
pub struct SendEmailJob {
    pub template: EmailTemplate,
    pub recipients: Vec<String>,
}

lazy_static! {
    static ref OUTBOUND_EMAIL_SENDER: String =
        env::var("OUTBOUND_EMAIL_SENDER").expect("OUTBOUND_EMAIL_SENDER must be set");
    static ref SENDGRID_API_KEY: String =
        env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
}

#[async_trait]
impl Runnable for SendEmailJob {
    async fn run(&self) -> Result<String, JobError> {
        // TODO: validate input
        // TODO: populate templates
        // TODO: send request to SendGrid (via external-service-proxy Lambda)
        info!(
            "sending email to {:#?} (template: {:#?})",
            self.recipients, self.template
        );
        populate_email_template()?;
        let lambda_client = LambdaClient::new(Region::UsEast1);
        let _response = lambda_client
            .invoke(InvocationRequest {
                client_context: None,
                function_name: "my_function_name".to_string(),
                invocation_type: None,
                log_type: None,
                payload: None,
                qualifier: None,
            })
            .await;
        Ok("DONE".to_string())
    }
}
