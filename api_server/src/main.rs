extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use std::env;
use warp::Filter;

use world_core::routes::cors_wrapper;
use world_core::utils::config::ROOT_DOMAIN_NAME;
use world_core::{db, routes};

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 10;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "world_core=debug,api_server=debug");
    }
    pretty_env_logger::init();

    let cors_origin_url = format!("https://{}", *ROOT_DOMAIN_NAME);

    // TODO: load DATABASE_URL with rusoto_sts
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url, DB_POOL_SIZE).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    let api = routes::api(pool.clone());
    let routes = api
        .with(warp::log("api_server::routing"))
        .with(cors_wrapper(&cors_origin_url));
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
