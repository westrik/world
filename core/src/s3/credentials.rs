use rusoto_core::Region;
use rusoto_credential::ChainProvider;
use rusoto_credential::{AutoRefreshingProvider, AwsCredentials, ProvideAwsCredentials};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};
use std::env;

use crate::errors::ApiError;
use crate::APPLICATION_NAME;

lazy_static! {
    static ref IAM_ROLE_ARN: String = env::var("IAM_ROLE_ARN").expect("IAM_ROLE_ARN must be set");
}

fn get_autorefreshing_assume_role_credentials_provider(
    region: Region,
) -> Result<impl ProvideAwsCredentials, ApiError> {
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
        .map_err(|_| ApiError::InternalError("Failed to assume role".to_string()))?)
}

pub async fn get_aws_credentials(region: Region) -> Result<AwsCredentials, ApiError> {
    let credentials = if cfg!(feature = "production") {
        let provider = get_autorefreshing_assume_role_credentials_provider(region)?;
        provider.credentials().await
    } else {
        ChainProvider::new().credentials().await
    };
    Ok(credentials
        .map_err(|_| ApiError::InternalError("Failed to authenticate with S3".to_string()))?)
}
