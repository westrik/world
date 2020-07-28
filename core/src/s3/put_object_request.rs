use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::PutObjectRequest;

pub fn generate_presigned_upload_url(
    bucket: String,
    key: String,
    file_size_bytes: i64,
    // credential_key: String,
    // credential_secret: String,
    // credential_token: String,
) -> String {
    // TODO: load AwsCredentials from a ChainProvider
    // let s3_client =
    let aws_credentials = AwsCredentials::new(
        "".to_string(),
        "".to_string(),
        Some("".to_string()),
        Default::default(),
    );
    let put_object_request = PutObjectRequest {
        bucket,
        key,
        content_length: Some(file_size_bytes),
        ..Default::default()
    };
    put_object_request.get_presigned_url(
        &Region::UsEast1,
        &aws_credentials,
        &PreSignedRequestOption {
            expires_in: Default::default(),
        },
    )
}

#[cfg(test)]
pub mod put_object_request_test {
    use super::generate_presigned_upload_url;

    #[test]
    fn generate_dummy_upload_url() {
        println!(
            "{}",
            generate_presigned_upload_url("fake-bucket".to_string(), "fake-name".to_string(), 1234,)
        );
    }
}
