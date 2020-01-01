use crate::db;
use crate::db::get_conn;
use crate::models::session::{Session, UiSession};
use crate::models::user::{NewUser, UiUser, User, UserQueryError};
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Json;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn sign_up(
    user: Json<NewUser>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    // TODO: return 400 for constraint errors
    let user = web::block(move || User::create(user.into_inner(), &get_conn(&pool).unwrap()))
        .await
        .map_err(|_| Error::from(HttpResponse::InternalServerError()))?;
    Ok(HttpResponse::Ok().json(UiUser::from(user)))
}

#[derive(Deserialize)]
pub struct SignInRequest {
    email_address: String,
    password: String,
}
#[derive(Serialize)]
pub struct SignInResponse {
    user: UiUser,
    session: UiSession,
}

fn run_sign_in(
    creds: Json<SignInRequest>,
    pool: &db::PgPool,
) -> Result<SignInResponse, UserQueryError> {
    let conn = get_conn(pool).unwrap();
    let user: User = User::find(creds.email_address.as_str(), creds.password.as_str(), &conn)?;
    let session: Session = Session::create(&conn, &user)?;
    Ok(SignInResponse {
        session: UiSession::from(session),
        user: UiUser::from(user),
    })
}

pub async fn sign_in(
    creds: Json<SignInRequest>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let resp: SignInResponse = web::block(move || run_sign_in(creds, &pool))
        .await
        .map_err(|_| HttpResponse::BadRequest().body("failed to login"))?;
    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize)]
pub struct Item {
    pub id: i32,
}

#[derive(Serialize)]
pub struct GetItemResponse {
    error: Option<String>,
    items: Option<Vec<Item>>,
}

pub async fn get_items(req: HttpRequest) -> Result<HttpResponse, Error> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        // TODO: query for items using auth header to lookup user ID

        Ok(HttpResponse::Ok().json(GetItemResponse {
            error: None,
            items: None,
        }))
    } else {
        Ok(HttpResponse::Unauthorized().json(GetItemResponse {
            error: Some("no token".to_string()),
            items: None,
        }))
    }
}
