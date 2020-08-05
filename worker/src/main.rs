#![feature(async_closure)]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate fallible_iterator;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use std::{env, thread};
use warp::Filter;

use world_core::db;

pub mod jobs;
pub mod routes;
pub mod run;
pub mod subscribe;

mod emails;

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 15;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "world_core=debug,world_worker=debug");
    }
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url, DB_POOL_SIZE).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    thread::spawn(async move || {
        let api = routes::worker_api(pool.clone());
        let routes = api.with(warp::log("worker::routing"));
        warp::serve(routes).run(([127, 0, 0, 1], 8090)).await;
    });

    if let Err(err) = subscribe::subscribe_to_jobs(database_url).await {
        error!("failed to subscribe to jobs: {:?}", err);
    }
}
