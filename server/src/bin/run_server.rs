extern crate log;

use actix_web::{http, middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use westrikworld_server::db;
use westrikworld_server::routes::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    // TODO: run pending migrations

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1"))
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("http://westrik.world:1234")
                    .allowed_origin("https://westrikworld.com")
                    .allowed_origin("https://staging.westrikworld.com")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .route("/sign-up", web::post().to(sign_up))
            .route("/sign-in", web::post().to(sign_in))
            .route("/item", web::post().to(create_item))
            .route("/item", web::get().to(get_items))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
