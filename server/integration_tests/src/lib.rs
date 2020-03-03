#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::runner)]

extern crate diesel;
#[allow(unused_imports)]
#[macro_use]
extern crate diesel_migrations;

#[cfg(test)]
mod db;
pub mod fixtures;
mod test_runner;

#[cfg(test)]
mod tasks;
