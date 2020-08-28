use chrono::{DateTime, Utc};
use serde_json::json;
use warp::http::{HeaderValue, Response, StatusCode};
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::auth::models::user::{ApiUserCreateSpec, User};
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::external_services::aws::cloudfront::generate_signed_cookie::{
    generate_cloudfront_access_cookies, CloudFrontAccessCookies,
};
use crate::jobs::enqueue_job::enqueue_job;
use crate::jobs::job_type::JobType;
use crate::utils::api_task::run_api_task;
use crate::utils::config::ROOT_DOMAIN_NAME;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    #[serde(rename = "emailAddress")]
    email_address: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthenticationResponse {
    user: Option<User>,
    session: Option<Session>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CloudfrontAuthenticationRequest {
    #[serde(rename = "userId")]
    user_api_id: String,
}

#[derive(Serialize)]
pub struct CloudfrontAuthenticationResponse {
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
}

fn run_sign_in(creds: SignInRequest, pool: &DbPool) -> Result<AuthenticationResponse, ApiError> {
    let conn = get_conn(pool).unwrap();
    let user: User = User::find(creds.email_address.as_str(), creds.password.as_str(), &conn)?;
    let session: Session = Session::create(&conn, &user)?;

    if let Ok(job) = enqueue_job(
        &conn,
        Some(user.id),
        JobType::SendEmail,
        Some(json!({
            "recipients": vec!["test@example.com"],
            "template": "LoginNotification"
        })),
    ) {
        info!(
            "enqueued email notification job [user_id={}][job_id={}]",
            user.id, job.id
        );
    } else {
        error!(
            "failed to enqueue email notification job [user_id={}]",
            user.id
        );
    }
    Ok(AuthenticationResponse {
        session: Some(session),
        user: Some(user),
        error: None,
    })
}

pub async fn sign_in(
    sign_in_request: SignInRequest,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("sign_in: email_address={}", sign_in_request.email_address);
    let response = run_api_task(move || run_sign_in(sign_in_request, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

fn run_cloudfront_authenticate(
    _creds: CloudfrontAuthenticationRequest,
    _pool: &DbPool,
) -> Result<CloudFrontAccessCookies, ApiError> {
    // TODO: generate path to user's cloudfront directory & verify session
    let path = "/";
    Ok(generate_cloudfront_access_cookies(path))
}

pub async fn cloudfront_authenticate(
    auth_request: CloudfrontAuthenticationRequest,
    _session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!(
        "cloudfront_authenticate: user_api_id={}",
        auth_request.user_api_id
    );
    let domain = format!("uploads.{}", *ROOT_DOMAIN_NAME);
    let path = "/"; // TODO: set this to user's cloudfront path

    let cookies = run_api_task(move || run_cloudfront_authenticate(auth_request, &db_pool)).await?;
    let cookie_headers = vec![
        ("CloudFront-Policy", cookies.encoded_policy),
        ("CloudFront-Signature", cookies.signature),
        ("CloudFront-Key-Pair-Id", cookies.key_pair_id),
    ];

    let mut response_builder = Response::builder();
    for header in cookie_headers {
        let value = HeaderValue::from_str(&format!(
            "{}={}; Domain={}; Path={}; Secure; HttpOnly",
            header.0, header.1, domain, path
        ))
        .map_err(|_| ApiError::InternalError("Could not create CloudFront cookie".to_string()))?;
        response_builder = response_builder.header(header.0, value);
    }
    Ok(response_builder
        .body(
            serde_json::to_string(&CloudfrontAuthenticationResponse {
                expires_at: cookies.session_expires_at,
            })
            .map_err(|_| {
                ApiError::InternalError(
                    "Could not serialize CloudFront authentication response".to_string(),
                )
            })?,
        )
        .map_err(|_| ApiError::InternalError("Error creating response".to_string()))?)
}

fn run_sign_up(
    create_spec: ApiUserCreateSpec,
    pool: &DbPool,
) -> Result<AuthenticationResponse, ApiError> {
    let user = User::create(create_spec, &get_conn(&pool).unwrap())?;
    Ok(AuthenticationResponse {
        user: Some(user),
        session: None, // TODO: create session upon sign-up
        error: None,
    })
}

pub async fn sign_up(
    create_spec: ApiUserCreateSpec,
    pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!(
        "sign_up: email_address={}, full_name={:?}",
        create_spec.email_address, create_spec.full_name
    );
    let response = run_api_task(move || run_sign_up(create_spec, &pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}
