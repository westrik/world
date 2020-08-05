use std::env;

use async_trait::async_trait;
use world_core::jobs::errors::JobError;

pub mod dummy_job;
pub mod send_email;

lazy_static! {
    pub static ref SERVICE_PROXY_LAMBDA_ARN: String =
        env::var("SERVICE_PROXY_LAMBDA_ARN").expect("SERVICE_PROXY_LAMBDA_ARN must be set");
}

#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<String, JobError>;
}
