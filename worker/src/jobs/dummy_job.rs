use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;

#[derive(Deserialize)]
pub struct DummyJob {}

impl Runnable for DummyJob {
    fn run(&self) -> Result<String, JobError> {
        info!("Running dummy job");
        Ok("DONE".to_string())
    }
}
