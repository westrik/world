use crate::db;
use crate::db::get_conn;
use crate::models::user::{NewUser, User};
use actix_web::{error, web, Error, HttpResponse, Responder};

pub async fn sign_up(body: web::Bytes, pool: web::Data<db::PgPool>) -> Result<HttpResponse, Error> {
    let new_user_json = serde_json::from_slice::<NewUser>(&body);
    match new_user_json {
        Ok(user) => {
            let new_user = NewUser {
                email_address: user.email_address,
                full_name: user.full_name,
                password: user.password,
            };
            // TODO: return 400 for constraint errors
            let user = User::create(new_user, &get_conn(&pool).unwrap())
                .map_err(|_| Error::from(HttpResponse::InternalServerError()))?;
            Ok(HttpResponse::Ok().json(user))
        }
        Err(_) => Err(error::ErrorBadRequest("Invalid JSON")),
    }
}

pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub async fn delete_users(pool: web::Data<db::PgPool>) -> impl Responder {
    User::delete_all(&get_conn(&pool).unwrap());
    HttpResponse::Ok().body("all users deleted")
}
