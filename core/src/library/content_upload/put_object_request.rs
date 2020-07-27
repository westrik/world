use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::PutObjectRequest;

pub fn generate_presigned_upload_url(
    bucket: String,
    key: String,
    credential_key: String,
    credential_secret: String,
    credential_token: String,
    // _file_size_bytes: Option<i32>
) -> String {
    let aws_credentials = AwsCredentials::new(
        credential_key,
        credential_secret,
        Some(credential_token),
        Default::default(),
    );
    let put_object_request = PutObjectRequest {
        bucket: bucket,
        key: key,
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
            generate_presigned_upload_url(
                "test-bucket".to_string(),
                "test.txt".to_string(),
                "FAKE-KEY".to_string(),
                "FAKE-SECRET".to_string(),
                "FAKE-TOKEN".to_string()
            )
        );
    }
}
