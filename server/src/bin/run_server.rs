#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use std::env;
use warp::Filter;

use westrikworld_server::{db, routes};

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

    let cors = warp::cors()
        .allow_origin(cors_origin_url.as_str())
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let api = routes::api(pool.clone());
    let routes = api.with(warp::log("run_server")).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
