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
use std::env;

use world_core::db;

pub mod jobs;
pub mod run;
pub mod subscribe;

mod emails;
mod media_transforms;
mod sites;

embed_migrations!("../core/migrations");

const DB_POOL_SIZE: u32 = 15;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "world_core=debug,world_worker=debug");
    }
    pretty_env_logger::init();

    // TODO: load DATABASE_URL with rusoto_sts
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url, DB_POOL_SIZE).expect("Failed to create pool");

    let conn = db::get_conn(&pool).unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();

    subscribe::subscribe_to_jobs(database_url, &pool)
        .await
        .unwrap()
}
