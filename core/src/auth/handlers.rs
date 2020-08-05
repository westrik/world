use serde_json::json;
use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::auth::models::user::{ApiUserCreateSpec, User};
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::jobs::enqueue_job::enqueue_job;
use crate::jobs::job_type::JobType;
use crate::utils::api_task::run_api_task;

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
