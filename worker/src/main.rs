extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate fallible_iterator;
extern crate postgres;

use dotenv::dotenv;
use std::{env, thread};
use warp::Filter;

use westrikworld_core::db;

mod routes;
mod subscribe;

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 15;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "westrikworld_core=debug,worker=debug");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url, DB_POOL_SIZE).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    thread::spawn(move || {
        subscribe::subscribe_to_jobs(database_url);
    });

    let api = routes::worker_api(pool.clone());
    let routes = api.with(warp::log("worker::routing"));
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}
