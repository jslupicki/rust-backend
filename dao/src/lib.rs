extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate monitor;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha3;

use diesel::QueryResult;

pub use connection::{initialize_db, get_connection};
pub use models::*;

#[cfg(test)]
mod common_for_tests;
mod connection;
mod employees_dao;
mod models;
mod schema;
mod users_dao;
mod salaries_dao;
mod contacts_dao;
mod base_dao;

pub fn create_user(new_user: &NewUser) -> QueryResult<User> {
    let conn = get_connection();
    users_dao::create_user(new_user, &conn)
}

pub fn update_user(user: &User) -> QueryResult<User> {
    let conn = get_connection();
    users_dao::update_user(user, &conn)
}

pub fn get_users() -> Vec<User> {
    let conn = get_connection();
    users_dao::get_users(&conn)
}

pub fn get_user(id: i32) -> Option<User> {
    let conn = get_connection();
    users_dao::get_user(id, &conn)
}

pub fn validate_user(username: &String, password: &String) -> bool {
    let conn = get_connection();
    users_dao::validate_user(username, password, &conn)
}
