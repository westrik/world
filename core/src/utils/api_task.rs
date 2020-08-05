use tokio::task::spawn_blocking;

use crate::errors::ApiError;

pub async fn run_api_task<R, F>(f: F) -> Result<R, ApiError>
where
    F: FnOnce() -> Result<R, ApiError> + Send + 'static,
    R: Send + 'static,
{
    spawn_blocking(f)
        .await
        .map_err(ApiError::InternalRuntimeError)?
}
