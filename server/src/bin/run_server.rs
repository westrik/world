#[macro_use]
extern crate log;

use std::{env, io};

use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, App, HttpServer}; // web,
use dotenv::dotenv;

use timeline_server::db;
use timeline_server::route;

fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let app = move || {
        debug!("constructing the app");

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                route::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, route::bad_request)
            .handler(http::StatusCode::NOT_FOUND, route::not_found);

        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(error_handlers)
            //.service(web::resource("/").route(web::get().to_async(route::index)))
            //            .service(web::resource("/todo").route(web::post().to_async(api::create)))
            //            .service(
            //                web::resource("/todo/{id}").route(web::post().to_async(api::update)),
            //            )
            .service(fs::Files::new("/static", "static/"))
    };

    debug!("starting server");
    HttpServer::new(app).bind("localhost:8088")?.run()
}
