#[macro_use]
extern crate diesel_migrations;

extern crate log;

use dotenv::dotenv;
use std::env;

use westrikworld_server::db;
use westrikworld_server::routes::*;

embed_migrations!();

fn main() -> () {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

//    HttpServer::new(move || {
//        App::new()
//            .data(pool.clone())
//            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1"))
//            .wrap(Logger::default())
//            .wrap(
//                Cors::new()
//                    .allowed_origin("http://westrik.world:1234")
//                    .allowed_origin("https://westrikworld.com")
//                    .allowed_origin("https://staging.westrikworld.com")
//                    .allowed_methods(vec!["GET", "POST"])
//                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//                    .allowed_header(http::header::CONTENT_TYPE)
//                    .max_age(3600)
//                    .finish(),
//            )
//            .route("/", web::get().to(|| async { "OK" }))
//            .route("/sign-up", web::post().to(sign_up))
//            .route("/sign-in", web::post().to(sign_in))
//            .route("/item", web::post().to(create_item))
//            .route("/item", web::get().to(get_items))
//    })
//    .bind("127.0.0.1:8080")?
//    .run()
//    .await
}
