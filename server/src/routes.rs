use crate::db;
use crate::db::get_conn;
use crate::models::user::{NewUser, UiUser, User};
use actix_web::{web, Error, HttpResponse};

pub async fn sign_up(
    user: web::Json<NewUser>,
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

pub async fn sign_in(
    creds: web::Json<SignInRequest>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let user: User = web::block(move || {
        User::find(
            creds.email_address.as_str(),
            creds.password.as_str(),
            &get_conn(&pool).unwrap(),
        )
    })
    .await
    .map_err(|_| Error::from(HttpResponse::BadRequest()))?;
    Ok(HttpResponse::Ok().json(UiUser::from(user)))
}

pub async fn delete_users(pool: web::Data<db::PgPool>) -> Result<HttpResponse, Error> {
    web::block(move || User::delete_all(&get_conn(&pool).unwrap()))
        .await
        .map_err(|_| Error::from(HttpResponse::InternalServerError()))?;
    Ok(HttpResponse::Ok().body("all users deleted"))
}
