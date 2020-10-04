use chrono::{DateTime, Utc};
use warp::http::{HeaderValue, Response, StatusCode};
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::external_services::aws::cloudfront::generate_signed_cookie::{
    generate_cloudfront_access_data, CloudFrontAccessData,
};
use crate::resource_identifier::split_resource_identifier;
use crate::utils::api_task::run_api_task;
use crate::utils::config::{MEDIA_DOMAIN_NAME, ROOT_DOMAIN_NAME};

#[derive(Deserialize)]
pub struct ApiUserCreateSpec {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub password: String,
}
#[derive(Serialize)]
pub struct AuthenticationResponse {
    user: Option<User>,
    session: Option<Session>,
    error: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    #[serde(rename = "emailAddress")]
    email_address: String,
    password: String,
}

fn run_sign_in(creds: SignInRequest, pool: &DbPool) -> Result<AuthenticationResponse, ApiError> {
    let conn = get_conn(pool).unwrap();
    let user: User = User::find(creds.email_address.as_str(), creds.password.as_str(), &conn)?;
    let session: Session = Session::create(&conn, &user)?;

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

#[derive(Debug, Deserialize)]
pub struct CloudfrontAuthenticationRequest {}

#[derive(Serialize)]
pub struct CloudfrontAuthenticationResponse {
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,
}

fn run_cloudfront_authenticate(
    session: &Session,
    pool: &DbPool,
) -> Result<CloudFrontAccessData, ApiError> {
    let user = session.get_user(&get_conn(pool).unwrap())?;
    let split_user_api_id = split_resource_identifier(&user.api_id);

    let path = format!("/{}/", split_user_api_id);
    let resource = format!("https://{}/{}/*", *MEDIA_DOMAIN_NAME, split_user_api_id);
    Ok(generate_cloudfront_access_data(&path, &resource))
}

pub async fn cloudfront_authenticate(
    _auth_request: CloudfrontAuthenticationRequest,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("cloudfront_authenticate: user_id={}", session.user_id);
    let cloudfront_access =
        run_api_task(move || run_cloudfront_authenticate(&session, &db_pool)).await?;
    let cookie_headers = vec![
        ("CloudFront-Policy", cloudfront_access.encoded_policy),
        ("CloudFront-Signature", cloudfront_access.signature),
        ("CloudFront-Key-Pair-Id", cloudfront_access.key_pair_id),
    ];

    let mut response_builder = Response::builder();
    for header in cookie_headers {
        let value = HeaderValue::from_str(&format!(
            "{}={}; Domain={}; Path={}; Secure; SameSite=Lax; HttpOnly",
            header.0, header.1, *ROOT_DOMAIN_NAME, cloudfront_access.path
        ))
        .map_err(|_| ApiError::InternalError("Could not create CloudFront cookie".to_string()))?;
        response_builder = response_builder.header("Set-Cookie", value);
    }
    Ok(response_builder
        .body(
            serde_json::to_string(&CloudfrontAuthenticationResponse {
                expires_at: cloudfront_access.session_expires_at,
            })
            .map_err(|_| {
                ApiError::InternalError(
                    "Could not serialize CloudFront authentication response".to_string(),
                )
            })?,
        )
        .map_err(|_| ApiError::InternalError("Error creating response".to_string()))?)
}
