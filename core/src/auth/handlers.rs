use crate::auth::models::session::Session;
use crate::auth::models::user::{ApiUserCreateSpec, User};
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use warp::http::StatusCode;
use warp::Rejection;

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
    let response = run_sign_in(sign_in_request, &db_pool)?;
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

pub async fn sign_up(
    new_user: ApiUserCreateSpec,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!(
        "sign_up: email_address={}, full_name={:?}",
        new_user.email_address, new_user.full_name
    );
    let user = User::create(new_user, &get_conn(&db_pool).unwrap())?;
    Ok(warp::reply::with_status(
        warp::reply::json(&AuthenticationResponse {
            user: Some(user),
            session: None, // TODO: create session upon sign-up
            error: None,
        }),
        StatusCode::OK,
    ))
}
