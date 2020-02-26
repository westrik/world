extern crate argon2rs;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod resource_identifier;
pub mod routes;
pub mod schema;

// TODO: move DB operations to an async thread pool (tokio?)
pub mod auth;
pub mod notes;
pub mod tasks;

pub mod test_utils;

static API_VERSION: &str = "0.1.1";
