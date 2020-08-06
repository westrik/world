use async_trait::async_trait;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;

#[derive(Deserialize)]
pub struct DummyJob {}

#[async_trait]
impl Runnable for DummyJob {
    async fn run(&self) -> Result<String, JobError> {
        info!("Running dummy job");
        Ok("DONE".to_string())
    }
}
