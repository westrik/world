use crate::db::{get_conn, PgPool};
use crate::models::session::{Session, UiSession};
use crate::models::user::{NewUser, UiUser, User, UserQueryError};
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    email_address: String,
    password: String,
}
#[derive(Serialize)]
pub struct SignInResponse {
    user: Option<UiUser>,
    session: Option<UiSession>,
    error: Option<String>,
}

fn run_sign_in(creds: SignInRequest, pool: &PgPool) -> Result<SignInResponse, UserQueryError> {
    let conn = get_conn(pool).unwrap();
    let user: User = User::find(creds.email_address.as_str(), creds.password.as_str(), &conn)?;
    let session: Session = Session::create(&conn, &user)?;
    Ok(SignInResponse {
        session: Some(UiSession::from(session)),
        user: Some(UiUser::from(user)),
        error: None,
    })
}

pub async fn sign_in(
    sign_in_request: SignInRequest,
    db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("sign_in: email_address={}", sign_in_request.email_address);

    Ok(match run_sign_in(sign_in_request, &db_pool) {
        Ok(response) => warp::reply::with_status(warp::reply::json(&response), StatusCode::OK),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&SignInResponse {
                session: None,
                user: None,
                error: Some("Failed to login".to_string()),
            }),
            StatusCode::BAD_REQUEST,
        ),
    })
}

#[derive(Serialize)]
pub struct SignUpResponse {
    // TODO: session: Option<UiSession>,
    user: Option<UiUser>,
    error: Option<String>,
}

pub async fn sign_up(new_user: NewUser, db_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "sign_up: email_address={}, full_name={:?}",
        new_user.email_address, new_user.full_name
    );

    Ok(match User::create(new_user, &get_conn(&db_pool).unwrap()) {
        Ok(user) => warp::reply::with_status(
            warp::reply::json(&SignUpResponse {
                user: Some(UiUser::from(user)),
                error: None,
            }),
            StatusCode::OK,
        ),
        Err(_) => warp::reply::with_status(
            warp::reply::json(&SignUpResponse {
                user: None,
                error: Some("Failed to create user".to_string()),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    })
}
