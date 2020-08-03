use world_core::jobs::errors::JobError;

pub mod dummy_job;
pub mod send_email;

pub trait Runnable {
    fn run(&self) -> Result<String, JobError>;
}
