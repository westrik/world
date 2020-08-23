use serde_json::json;
use std::{env, time::SystemTime};

lazy_static! {
    static ref CLOUDFRONT_KEYPAIR_ID: String =
        env::var("CLOUDFRONT_KEYPAIR_ID").expect("CLOUDFRONT_KEYPAIR_ID must be set");
    static ref CLOUDFRONT_PRIVATE_KEY: String =
        env::var("CLOUDFRONT_PRIVATE_KEY").expect("CLOUDFRONT_PRIVATE_KEY must be set");
}

fn cookie_expires_at_epoch_time() -> u64 {
    let current_epoch_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX epoch!"),
    };
    // The cookie should expire one hour (3600 seconds) from now.
    current_epoch_time + 3600
}

pub fn create_policy(path: &str, expires_at_epoch_time: u64) -> String {
    json!({
        "Statement": [
            {
                "Resource": path,
                "Condition": {
                    "DateLessThan": {
                        "AWS:EpochTime": expires_at_epoch_time,
                    },
                },
            },
        ],
    })
    .to_string()
}

fn strip_unsupported_characters(input: String) -> String {
    input.replace("+", "-").replace("=", "_").replace("/", "~")
}

pub fn encode_policy(policy: &str) -> String {
    strip_unsupported_characters(base64::encode(policy))
}

pub fn sign_policy(policy: &str, _private_key: &str) -> String {
    let encoded_policy = encode_policy(policy);
    let signature = encoded_policy;
    // TODO: actually sign it w/ RSA
    strip_unsupported_characters(signature)
}

pub fn generate_cloudfront_access_cookies(path: &str) -> serde_json::Value {
    let policy = create_policy(path, cookie_expires_at_epoch_time());
    let encoded_policy = encode_policy(&policy);
    let signature = sign_policy(&policy, &*CLOUDFRONT_PRIVATE_KEY);
    json!({
        "CloudFront-Policy": encoded_policy,
        "CloudFront-Signature": signature,
        "CloudFront-Key-Pair-Id": *CLOUDFRONT_KEYPAIR_ID
    })
}

#[cfg(test)]
pub mod cloudfront_signed_cookie_generation {
    use super::*;

    #[test]
    fn test_create_policy() {
        let expected_value = r#"{"Statement":[{"Condition":{"DateLessThan":{"AWS:EpochTime":1234}},"Resource":"https://example.com/hello-world"}]}"#;
        assert_eq!(
            expected_value,
            create_policy("https://example.com/hello-world", 1234)
        )
    }

    #[test]
    fn test_encode_policy() {
        let policy = create_policy("https://example.com/hello-world", 123456);
        let expected_encoded_policy = "eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTIzNDU2fX0sIlJlc291cmNlIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9oZWxsby13b3JsZCJ9XX0_";
        assert_eq!(expected_encoded_policy, encode_policy(&policy));
    }

    #[test]
    fn test_encode_and_sign_policy() {
        let policy = create_policy("https://example.com/hello-world", 123456);
        let private_key = "abcdefghijklmnop";
        let expected_encoded_value = "eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTIzNDU2fX0sIlJlc291cmNlIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9oZWxsby13b3JsZCJ9XX0_";
        assert_eq!(expected_encoded_value, sign_policy(&policy, private_key));
    }
}
