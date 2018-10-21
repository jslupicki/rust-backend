// TODO: remove when Diesel fix https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate log;
extern crate actix_web;
extern crate log4rs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

use actix_web::{server, App, HttpRequest};

pub mod models;
pub mod schema;
mod users_dao;

#[cfg(test)]
mod tests;

use models::{NewUser, User};
use schema::users;
use diesel::dsl::*;
use diesel::insert_into;
use users::dsl::*;

lazy_static! {
    static ref pool: Pool<ConnectionManager<SqliteConnection>> = create_connection_pool();
}

pub fn create_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn index(_req: &HttpRequest) -> &'static str {
    info!("Got request!");
    "Hello world!"
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

