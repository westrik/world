use rusoto_core::{HttpClient, Region};
use rusoto_credential::{AutoRefreshingProvider, ChainProvider, ProfileProvider};
use rusoto_s3::S3Client;
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};

use crate::errors::ApiError;
use crate::APPLICATION_NAME;

#[allow(dead_code)]
pub fn with_profile(region: Region, profile: impl Into<String>) -> Result<S3Client, ApiError> {
    let mut profile_provider = ProfileProvider::new()
        .map_err(|_| ApiError::InternalError("Failed to load AWS profile".to_string()))?;
    profile_provider.set_profile(profile);

    let chain_provider = ChainProvider::with_profile_provider(profile_provider);
    Ok(S3Client::new_with(
        HttpClient::new()
            .map_err(|_| ApiError::InternalError("Failed to connect to S3".to_string()))?,
        chain_provider,
        region,
    ))
}

pub fn with_assume_role(region: Region, role_arn: impl Into<String>) -> Result<S3Client, ApiError> {
    let sts_client = StsClient::new(region.clone());
    let assume_role_provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts_client,
        role_arn.into(),
        APPLICATION_NAME.to_owned(),
        None,
        Some(chrono::Duration::minutes(30)),
        None,
        None,
    );

    let auto_refreshing_assume_role_provider = AutoRefreshingProvider::new(assume_role_provider)
        .map_err(|_| ApiError::InternalError("Failed to authenticate with S3".to_string()))?;

    Ok(S3Client::new_with(
        HttpClient::new()
            .map_err(|_| ApiError::InternalError("Failed to connect to S3".to_string()))?,
        auto_refreshing_assume_role_provider,
        region,
    ))
}
