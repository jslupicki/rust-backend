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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha3;

use diesel::QueryResult;

pub use base_dao::{Crud, Searchable, SearchableByParent};
pub use connection::{get_connection, initialize_db, MIGRATIONS};
pub use employees_dao::EmployeeDTO;
pub use models::*;

mod base_dao;
#[cfg(test)]
mod common_for_tests;
mod connection;
mod contacts_dao;
mod employees_dao;
mod models;
mod salaries_dao;
mod schema;
mod users_dao;

pub fn create_user(new_user: &NewUser) -> QueryResult<User> {
    let mut conn = get_connection();
    users_dao::create_user(new_user, &mut conn)
}

pub fn update_user(user: &User) -> QueryResult<User> {
    let mut conn = get_connection();
    users_dao::update_user(user, &mut conn)
}

pub fn delete_user(user: &User) -> QueryResult<usize> {
    let mut conn = get_connection();
    users_dao::delete_user(user, &mut conn)
}

pub fn get_users() -> Vec<User> {
    let mut conn = get_connection();
    users_dao::get_users(&mut conn)
}

pub fn get_user(id: i32) -> Option<User> {
    let mut conn = get_connection();
    users_dao::get_user(id, &mut conn)
}

pub fn validate_user(username: &String, password: &String) -> Option<User> {
    let mut conn = get_connection();
    users_dao::validate_user(username, password, &mut conn)
}
