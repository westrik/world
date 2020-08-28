use std::env;

lazy_static! {
    pub static ref CLOUDFRONT_KEYPAIR_ID: String = env::var("CLOUDFRONT_KEYPAIR_ID").expect("CLOUDFRONT_KEYPAIR_ID must be set");
    pub static ref CLOUDFRONT_PRIVATE_KEY: String = env::var("CLOUDFRONT_PRIVATE_KEY").expect("CLOUDFRONT_PRIVATE_KEY must be set");
    pub static ref CONTENT_BUCKET_NAME: String = env::var("CONTENT_BUCKET_NAME").expect("CONTENT_BUCKET_NAME must be set");
    pub static ref IAM_ROLE_ARN: String = env::var("IAM_ROLE_ARN").expect("IAM_ROLE_ARN must be set");
    pub static ref OUTBOUND_EMAIL_SENDER: String = env::var("OUTBOUND_EMAIL_SENDER").expect("OUTBOUND_EMAIL_SENDER must be set");
    pub static ref PASSWORD_HASH_SALT: String = env::var("PASSWORD_HASH_SALT").expect("PASSWORD_HASH_SALT must be set");
    pub static ref ROOT_DOMAIN_NAME: String = env::var("ROOT_DOMAIN_NAME").expect("ROOT_DOMAIN_NAME must be set");
    pub static ref SENDGRID_API_KEY: String = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
    // pub static ref SERVICE_PROXY_LAMBDA_ARN: String = env::var("SERVICE_PROXY_LAMBDA_ARN").expect("SERVICE_PROXY_LAMBDA_ARN must be set");
}
