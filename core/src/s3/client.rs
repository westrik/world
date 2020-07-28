use rusoto_core::Region;
use rusoto_credential::{AutoRefreshingProvider, ProvideAwsCredentials};
// use rusoto_credential::{ChainProvider, ProfileProvider};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};
use std::env;

use crate::errors::ApiError;
use crate::APPLICATION_NAME;

lazy_static! {
    static ref IAM_ROLE_ARN: String = env::var("IAM_ROLE_ARN").expect("IAM_ROLE_ARN must be set");
}

pub fn get_aws_credentials_provider(
    region: Region,
) -> Result<impl ProvideAwsCredentials, ApiError> {
    // TODO: use ChainProvider when running locally
    // #[allow(dead_code)]
    // pub fn with_profile(region: Region, profile: impl Into<String>) -> Result<S3Client, ApiError> {
    //     let mut profile_provider = ProfileProvider::new()
    //         .map_err(|_| ApiError::InternalError("Failed to load AWS profile".to_string()))?;
    //     profile_provider.set_profile(profile);
    //
    //     let chain_provider = ChainProvider::with_profile_provider(profile_provider);
    //     Ok(S3Client::new_with(
    //         HttpClient::new()
    //             .map_err(|_| ApiError::InternalError("Failed to connect to S3".to_string()))?,
    //         chain_provider,
    //         region,
    //     ))
    // }
    let sts_client = StsClient::new(region);
    let assume_role_provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts_client,
        (*IAM_ROLE_ARN).to_owned(),
        APPLICATION_NAME.to_owned(),
        None,
        Some(chrono::Duration::minutes(30)),
        None,
        None,
    );
    Ok(AutoRefreshingProvider::new(assume_role_provider)
        .map_err(|_| ApiError::InternalError("Failed to authenticate with S3".to_string()))?)
}
