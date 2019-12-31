#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use timeline_server::db;
use timeline_server::routes::*;
use actix_web::middleware::Logger;

/*
TODO(next):
 - add methods for password hashing + authenticating
 - hook up sign-in endpoint
 - 4XX/5XX status pages
DONE:
 - add user model
*/

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .route("/sign-up", web::post().to(sign_up))
            .route("/sign-in", web::post().to(sign_in))
            .route("/delete-users", web::delete().to(delete_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
