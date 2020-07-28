use crate::errors::ApiError;
use crate::s3::credentials::get_aws_credentials;
use rusoto_core::Region;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::PutObjectRequest;

pub async fn generate_presigned_upload_url(
    bucket: String,
    key: String,
    file_size_bytes: i64,
) -> Result<String, ApiError> {
    let credentials = get_aws_credentials(Region::UsEast1).await?;
    let put_object_request = PutObjectRequest {
        bucket,
        key,
        content_length: Some(file_size_bytes),
        ..Default::default()
    };
    Ok(put_object_request.get_presigned_url(
        &Region::UsEast1,
        &credentials,
        &PreSignedRequestOption {
            expires_in: Default::default(),
        },
    ))
}
