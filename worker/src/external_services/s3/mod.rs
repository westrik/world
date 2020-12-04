use rusoto_core::credential::StaticProvider;
use rusoto_core::{ByteStream, HttpClient, Region};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

use world_core::jobs::errors::JobError;

pub async fn put_object_with_custom_credentials(
    access_key_id: String,
    access_key_secret: String,
    bucket_name: String,
    key: &str,
    data: Vec<u8>,
) -> Result<String, JobError> {
    S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        StaticProvider::new_minimal(access_key_id, access_key_secret),
        Region::UsEast1,
    )
    .put_object(PutObjectRequest {
        bucket: bucket_name,
        key: key.to_string(),
        body: Some(ByteStream::from(data)),
        ..Default::default()
    })
    .await
    .map_err(|e| {
        JobError::InternalError(format!("Failed to upload file to S3 (error: {:#?})", e))
    })?;
    Ok("Object PUT succeeded".to_string())
}
