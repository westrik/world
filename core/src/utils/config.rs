use std::env;

macro_rules! config_flag {
    ($flag_name:ident) => {
        lazy_static! {
            pub static ref $flag_name: String = env::var(stringify!($flag_name)).expect(&format!(
                "Expected environment variable '{}' to be set",
                stringify!($flag_name)
            ));
        }
    };
}

config_flag!(CLOUDFRONT_KEYPAIR_ID);
config_flag!(CLOUDFRONT_PRIVATE_KEY);
config_flag!(CONTENT_BUCKET_NAME);
config_flag!(IAM_ROLE_ARN);
config_flag!(OUTBOUND_EMAIL_SENDER);
config_flag!(PASSWORD_HASH_SALT);
config_flag!(ROOT_DOMAIN_NAME);
config_flag!(SENDGRID_API_KEY);
config_flag!(SERVICE_PROXY_LAMBDA_ARN);

lazy_static! {
    pub static ref MEDIA_DOMAIN_NAME: String = format!("media.{}", *ROOT_DOMAIN_NAME);
}
