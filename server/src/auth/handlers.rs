use crate::db::PgPool;
use crate::models::user::NewUser;
use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    email_address: String,
    password: String,
}
//#[derive(Serialize)]
//pub struct SignInResponse {
//    user: UiUser,
//    session: UiSession,
//}

pub async fn sign_in(
    sign_in_request: SignInRequest,
    _db_pool: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    log::debug!("sign_in: {:?}", sign_in_request);

    Ok(StatusCode::OK)
}

pub async fn sign_up(new_user: NewUser, _db_pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    log::debug!("sign_up: {:?}", new_user);

    Ok(StatusCode::OK)
}
