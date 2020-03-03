use crate::auth::models::session::{ApiSession, Session};
use crate::auth::models::user::{ApiUser, ApiUserCreateSpec, User, UserQueryError};
use crate::db::{get_conn, DbPool};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct SignInRequest {
    emailAddress: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthenticationResponse {
    user: Option<ApiUser>,
    session: Option<ApiSession>,
    error: Option<String>,
}

fn run_sign_in(
    creds: SignInRequest,
    pool: &DbPool,
) -> Result<AuthenticationResponse, UserQueryError> {
    let conn = get_conn(pool).unwrap();
    let user: User = User::find(creds.emailAddress.as_str(), creds.password.as_str(), &conn)?;
    let session: Session = Session::create(&conn, &user)?;
    Ok(AuthenticationResponse {
        session: Some(ApiSession::from(session)),
        user: Some(ApiUser::from(user)),
        error: None,
    })
}

pub async fn sign_in(
    sign_in_request: SignInRequest,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("sign_in: email_address={}", sign_in_request.emailAddress);

    Ok(match run_sign_in(sign_in_request, &db_pool) {
        Ok(response) => warp::reply::with_status(warp::reply::json(&response), StatusCode::OK),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&AuthenticationResponse {
                session: None,
                user: None,
                error: Some("Failed to login".to_string()),
            }),
            StatusCode::BAD_REQUEST,
        ),
    })
}

pub async fn sign_up(
    new_user: ApiUserCreateSpec,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "sign_up: email_address={}, full_name={:?}",
        new_user.emailAddress, new_user.fullName
    );

    Ok(match User::create(new_user, &get_conn(&db_pool).unwrap()) {
        Ok(user) => warp::reply::with_status(
            warp::reply::json(&AuthenticationResponse {
                user: Some(ApiUser::from(user)),
                session: None, // TODO: create session upon sign-up
                error: None,
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&AuthenticationResponse {
                user: None,
                session: None,
                error: Some("Failed to create user".to_string()),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}
