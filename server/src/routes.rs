use crate::db;
use crate::db::get_conn;
use crate::models::user::{NewUser, User};
use actix_web::{error, web, Error, HttpResponse, Responder};

pub async fn sign_up(
    user: web::Json<NewUser>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    // TODO: return 400 for constraint errors
    let user = web::block(move || User::create(user.into_inner(), &get_conn(&pool).unwrap()))
        .await
        .map_err(|_| Error::from(HttpResponse::InternalServerError()))?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub async fn delete_users(pool: web::Data<db::PgPool>) -> impl Responder {
    User::delete_all(&get_conn(&pool).unwrap());
    HttpResponse::Ok().body("all users deleted")
}
