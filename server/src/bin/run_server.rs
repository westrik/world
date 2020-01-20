#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use std::env;

use westrikworld_server::db;
use westrikworld_server::routes::*;

embed_migrations!();

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        // Set `RUST_LOG=run_server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "run_server=info");
    }
    pretty_env_logger::init();

    let cors_origin_url = env::var("CORS_ORIGIN_URL").expect("CORS_ORIGIN_URL must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    // TODO:
    //  - hook up warp server
    //  - configure CORS headers (take URL as env var)
    //  - async route handler (handle logging, db pool)
    //  - after filter to add version header

    // each module should export a list of routes
    // then the routes should be chained together with .or()

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
