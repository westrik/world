use rusoto_s3::{GetObjectOutput, GetObjectRequest, S3Client, S3};

use crate::errors::ApiError;
use crate::external_services::aws::REGION;

pub async fn get_object(bucket: String, key: &str) -> Result<GetObjectOutput, ApiError> {
    Ok(S3Client::new(REGION)
        .get_object(GetObjectRequest {
            bucket,
            key: key.to_string(),
            ..Default::default()
        })
        .await
        .map_err(|e| {
            ApiError::InternalError(format!("Failed to fetch uploaded file (error: {:#?})", e))
        })?)
}
