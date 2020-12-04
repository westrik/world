use chrono::Duration;
use rusoto_core::ByteStream;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use crate::errors::ApiError;
use crate::external_services::aws::{credentials::get_aws_credentials, REGION};

pub async fn put_object(bucket: String, key: &str, data: Vec<u8>) -> Result<String, ApiError> {
    S3Client::new(REGION)
        .put_object(PutObjectRequest {
            bucket,
            key: key.to_string(),
            body: Some(ByteStream::from(data)),
            ..Default::default()
        })
        .await
        .map_err(|e| {
            ApiError::InternalError(format!("Failed to upload file to S3 (error: {:#?})", e))
        })?;
    Ok("".to_string())
}

pub async fn generate_presigned_upload_url(
    bucket: String,
    key: String,
    file_size_bytes: i64,
) -> Result<String, ApiError> {
    let credentials = get_aws_credentials(REGION).await?;
    let put_object_request = PutObjectRequest {
        bucket,
        key,
        content_length: Some(file_size_bytes),
        ..Default::default()
    };
    Ok(put_object_request.get_presigned_url(
        &REGION,
        &credentials,
        &PreSignedRequestOption {
            expires_in: Duration::minutes(30).to_std().unwrap(),
        },
    ))
}
