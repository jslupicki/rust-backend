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
extern crate sha3;

use std::env;

use diesel::migration::MigrationError;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use models::User;

mod models;
mod schema;
mod users_dao;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<SqliteConnection>> = create_connection_pool();
}

fn create_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_users() -> Vec<User> {
    let conn = POOL.get().unwrap();
    users_dao::get_users(&conn)
}

pub fn validate_user(username: &String, password: &String) -> bool {
    let conn = POOL.get().unwrap();
    users_dao::validate_user(username, password, &conn)
}

pub fn initialize_db() -> Result<(), RunMigrationsError> {
    let conn = POOL.get().unwrap();
    users_dao::initialize_db(&conn)
}
