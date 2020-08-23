// use rusoto_core::Region;
use serde_json::json;

// use crate::external_services::aws::credentials::get_aws_credentials;
use std::time::SystemTime;

fn cookie_expires_at_epoch_time() -> u64 {
    let current_epoch_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX epoch!"),
    };
    // The cookie should expire one hour (3600 seconds) from now.
    current_epoch_time + 3600
}

pub async fn generate_signed_cookie(path: &str) -> String {
    // let credentials = get_aws_credentials(Region::UsEast1).await?;
    // let secret_access_key = credentials.aws_secret_access_key();

    create_cookie(path, cookie_expires_at_epoch_time())

    // TODO: create JSON object
    // TODO: hash & sign the JSON
    //  - can we use rusoto_signature for the fancy stuff?
}

pub fn create_cookie(path: &str, expires_at_epoch_time: u64) -> String {
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

pub fn encode_cookie_value(cookie_value: &str) -> String {
    base64::encode(cookie_value)
        .replace("+", "-")
        .replace("=", "_")
        .replace("/", "~")
}

#[cfg(test)]
pub mod cloudfront_signed_cookie_generation {
    use super::*;

    #[test]
    fn test_create_cookie_value() {
        let expected_value = r#"{"Statement":[{"Condition":{"DateLessThan":{"AWS:EpochTime":1234}},"Resource":"https://example.com/hello-world"}]}"#;
        assert_eq!(
            expected_value,
            create_cookie("https://example.com/hello-world", 1234)
        )
    }

    #[test]
    fn test_encode_cookie_value() {
        let cookie_value = create_cookie("https://example.com/hello-world", 123456);
        let encoded_value = encode_cookie_value(&cookie_value);
        let expected_encoded_value = "eyJTdGF0ZW1lbnQiOlt7IkNvbmRpdGlvbiI6eyJEYXRlTGVzc1RoYW4iOnsiQVdTOkVwb2NoVGltZSI6MTIzNDU2fX0sIlJlc291cmNlIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9oZWxsby13b3JsZCJ9XX0_";
        assert_eq!(expected_encoded_value, encoded_value);
    }

    #[test]
    fn test_sign_encoded_cookie_value() {}
}

/*
#!/usr/bin/env python

"""
Copyright (C) 2017 Matt Sullivan
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
Creates and signs cookies that provide access to CloudFront objects
Based on: https://aws.amazon.com/premiumsupport/knowledge-center/cf-signed-cookies-s3-origin/
requirements:
>=Python3.5
cryptography>=1.7.1
"""

import time
import json
import base64

from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import padding


def _replace_unsupported_chars(some_str):
    """Replace unsupported chars: '+=/' with '-_~'"""
    return some_str.replace("+", "-") \
        .replace("=", "_") \
        .replace("/", "~")


def _in_an_hour():
    """Returns a UTC POSIX timestamp for one hour in the future"""
    return int(time.time()) + (60*60)


def rsa_signer(message, key):
    """
    Based on https://boto3.readthedocs.io/en/latest/reference/services/cloudfront.html#examples
    """
    private_key = serialization.load_pem_private_key(
        key,
        password=None,
        backend=default_backend()
    )
    signer = private_key.signer(padding.PKCS1v15(), hashes.SHA1())
    signer.update(message)
    return signer.finalize()


def generate_policy_cookie(url):
    """Returns a tuple: (policy json, policy base64)"""

    policy_dict = {
        "Statement": [
            {
                "Resource": url,
                "Condition": {
                    "DateLessThan": {
                        "AWS:EpochTime": _in_an_hour()
                    }
                }
            }
        ]
    }

    # Using separators=(',', ':') removes seperator whitespace
    policy_json = json.dumps(policy_dict, separators=(",", ":"))

    policy_64 = str(base64.b64encode(policy_json.encode("utf-8")), "utf-8")
    policy_64 = _replace_unsupported_chars(policy_64)
    return policy_json, policy_64


def generate_signature(policy, key):
    """Creates a signature for the policy from the key, returning a string"""
    sig_bytes = rsa_signer(policy.encode("utf-8"), key)
    sig_64 = _replace_unsupported_chars(str(base64.b64encode(sig_bytes), "utf-8"))
    return sig_64


def generate_cookies(policy, signature, cloudfront_id):
    """Returns a dictionary for cookie values in the form 'COOKIE NAME': 'COOKIE VALUE'"""
    return {
        "CloudFront-Policy": policy,
        "CloudFront-Signature": signature,
        "CloudFront-Key-Pair-Id": cloudfront_id
    }


def generate_curl_cmd(url, cookies):
    """Generates a cURL command (use for testing)"""
    curl_cmd = "curl -v"
    for k, v in cookies.items():
        curl_cmd += " -H 'Cookie: {}={}'".format(k, v)
    curl_cmd += " {}".format(url)
    return curl_cmd


def generate_signed_cookies(url, cloudfront_id, key):
    policy_json, policy_64 = generate_policy_cookie(url)
    signature = generate_signature(policy_json, key)
    return generate_cookies(policy_64, signature, cloudfront_id)


if __name__ == "__main__":
    import argparse

    # Parse the command line args
    parser = argparse.ArgumentParser(description="Generates signed cookies for Amazon CloudFront")
    parser.add_argument("url", type=str, help="URL to sign")
    parser.add_argument("key", type=str, help="private key to use to sign the cookie")
    parser.add_argument("id", type=str, help="CloudFront id")
    args = parser.parse_args()

    with open(args.key, "rb") as f:
        cookies = generate_signed_cookies(args.url, args.id, f.read())

    print(generate_curl_cmd(args.url, cookies))
 */
