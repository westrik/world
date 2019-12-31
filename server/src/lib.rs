#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate argon2rs;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
