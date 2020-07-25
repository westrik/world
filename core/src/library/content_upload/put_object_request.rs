use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::PutObjectRequest;

pub fn generate_presigned_upload_url(_file_size_bytes: i32) -> String {
    let aws_credentials = AwsCredentials::new(
        "fake",
        "fake",
        Some("fake-token".to_string()),
        Default::default(),
    );
    let put_object_request = PutObjectRequest {
        bucket: "westrik-test-bucket-asdeflkj".to_string(),
        key: "fake-key".to_string(),
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
