use rsa::{Hash, PaddingScheme, RSAPrivateKey};
use serde_json::json;
use sha1::{Digest, Sha1};
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
                "Condition": {
                    "DateLessThan": {
                        "AWS:EpochTime": expires_at_epoch_time,
                    },
                },
                "Resource": path,
            },
        ],
    })
    .to_string()
}

fn swap_unsupported_characters(input: String) -> String {
    input.replace("+", "-").replace("=", "_").replace("/", "~")
}

pub fn encode_policy(policy: &str) -> String {
    base64::encode(policy)
}

pub fn sign_policy(policy: &str, private_key: &str) -> String {
    let encoded_policy = encode_policy(policy);
    let der_encoded_private_key = private_key
        .lines()
        .filter(|line| !line.starts_with('-'))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded_private_key)
        .expect("Failed to base64-decode CloudFront private key");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes)
        .expect("Failed to parse PKCS1 private key for CloudFront");
    let digest = Sha1::digest(encoded_policy.as_bytes()).to_vec();
    let signature = private_key
        .sign(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA1)), &digest)
        .unwrap();
    base64::encode(signature)
}

pub fn generate_cloudfront_access_cookies(path: &str) -> serde_json::Value {
    let policy = create_policy(path, cookie_expires_at_epoch_time());
    let encoded_policy = swap_unsupported_characters(encode_policy(&policy));
    let signature = swap_unsupported_characters(sign_policy(&policy, &*CLOUDFRONT_PRIVATE_KEY));
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
        let expected_encoded_policy = r#"eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTIzNDU2fX0sIlJlc291cmNlIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9oZWxsby13b3JsZCJ9XX0="#;
        assert_eq!(expected_encoded_policy, encode_policy(&policy));
    }

    #[test]
    fn test_encode_and_sign_policy() {
        let policy = create_policy("https://example.com/hello-world", 123456);
        let private_key = r#"-----BEGIN RSA PRIVATE KEY-----
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
-----END RSA PRIVATE KEY-----
"#;
        let expected_signature = "h4DirR4cuuCCvlg5jzfzYXspap0iPmDsqbgdIGhtf1BrZSTO7j0qdmDY5I1fTITg+ixIIne+7AjsLTYpTNZE3wOgzK/ys42RkndbTPzkopSateqEMpMN2RmaCTQpYBBZvnkC9AYrUMivSBoE8OMBdqIwMbi7HjRzOLHEF9kJYD+hko0yMjtw5goJv/o4EmEj1NhRXqaAGxVX+nVumQfrUhJq1XnyYKViZk75+gfmWaFxAC/pNRptDz1tdEs58NkKFBysBLZ35J9u+CexKC6hgQMeUOJKQWHxAdV9qmHEgtuECAXXIPYhqCOxIBI7kPY/Om7YkKob6Rj3PZguOnpojobrmdeAbOuP+B1lsRu1znafVB4zUgZcPZtfTSlTUJcZV0rKA1CNIjO1EJCWtQhKt2kQ0oz8dtfi7llZeF97+nU08lN/8XcYXVqyEKz5WmQK/ZPlznTAEV9pBl3qhtDtVrrvfM5oeuLoTe2q4pmNCHQxSPMVTPl7QhA/c1TjMHZHMc8G16GGp/I7NpI2fG7Xn80XxzzcBD+RdiQ/v9vP8cbZRB/aCCQzzEqbT7wgnLBjklVdgDaP0Ry7gVtdi9WIG8b5x7ktLZ6NbwwwUTZBGjkzKTmI78UXbK0cyU5SAUBVPKIHwKJuPSYF8uSgI7jiwOVf510h6Fv5hlOvXeicQbE=";
        assert_eq!(expected_signature, sign_policy(&policy, private_key));
    }
}
