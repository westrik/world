extern crate argon2rs;
#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod errors;
pub mod jobs;
pub mod resource_identifier;
pub mod routes;
pub mod s3;
pub mod schema;
pub mod utils;

// TODO: move DB operations to an async thread pool (tokio?)
pub mod auth;
pub mod library;
pub mod notes;
pub mod tasks;

pub static APPLICATION_NAME: &str = "westrikworld";
pub static API_VERSION: &str = "0.1.23";

// Limit request body size to 2 MB
static MAX_CONTENT_LENGTH_BYTES: u64 = 1024 * 1024 * 2;
