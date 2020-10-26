use async_trait::async_trait;
use bytes::Bytes;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use serde_json::json;

use world_core::db::DbPool;
use world_core::jobs::errors::JobError;
use world_core::utils::config::SENDGRID_API_KEY;

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

#[async_trait]
impl Runnable for SendEmailJob {
    async fn run(&self, _: &DbPool, _: Option<i32>) -> Result<String, JobError> {
        // TODO: validate input
        info!(
            "sending email to {:#?} (template: {:#?})",
            self.recipients, self.template
        );
        let email_body = populate_email_template()?;
        let lambda_client = LambdaClient::new(Region::UsEast1);

        let payload_str = json!({
            "service": "SENDGRID",
            "serviceToken": (*SENDGRID_API_KEY),
            "requestType": "SEND_EMAIL",
            "requests": [
                {
                    "from": {"email": "no-reply@westrik.world"},
                    "subject": "Testing 1, 2, 3",
                    "content": [
                        // TODO: send plain-text version
                        // {
                        //     "type": "text/plain",
                        //     "value": "This is a test"
                        // },
                        {
                            "type": "text/html",
                            "value": email_body
                        }
                    ],
                    "personalizations": [
                        {
                            "to": [{"email": "matt@westrik.world"}]
                        }
                    ]
                }
            ]
        })
        .to_string();
        let payload = Some(Bytes::from(payload_str));
        let _response = lambda_client
            .invoke(InvocationRequest {
                function_name: "service_proxy".to_string(),
                invocation_type: Some("RequestResponse".to_string()),
                payload,
                client_context: None,
                log_type: None,
                qualifier: None,
            })
            .await;
        Ok("DONE".to_string())
    }
}
