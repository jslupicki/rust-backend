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

use self::models::{NewUser, User};
use self::schema::users;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    static TEST_DB_NAME: &str = "test_db.sqlite3";

    embed_migrations!("./migrations");

    fn initialize_db() -> SqliteConnection {
        let _ = remove_file(TEST_DB_NAME); // Ignore error when there is no TEST_DB_NAME file
        let conn = SqliteConnection::establish(TEST_DB_NAME)
            .expect(&format!("Error connecting to {}", TEST_DB_NAME));
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
        conn
    }

    fn assert_user_count(expected: i64, conn: &SqliteConnection) {
        let user_count: i64 = users.select(count(id)).first(conn).unwrap();
        assert_eq!(user_count, expected);
    }

    #[test]
    fn crud_operations_on_user() {
        // Initialize
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = &initialize_db();

        // Check if DB is empty
        assert_user_count(0, conn);

        // Insert new_user
        let new_user = NewUser {
            username: "admin".to_string(),
            password: "admin_pass".to_string(),
            is_admin: true,
        };
        let rows_inserted = insert_into(users).values(&new_user).execute(conn);
        assert_eq!(Ok(1), rows_inserted);
        assert_user_count(1, conn);

        // Read user
        let users_in_db = users
            .filter(username.eq(&new_user.username))
            .limit(2)
            .load::<User>(conn)
            .expect("Error loading users");
        assert_eq!(1, users_in_db.len());
        let user = &users_in_db[0];
        assert_eq!(&new_user.username, &user.username);
        assert_eq!(&new_user.password, &user.password);
        assert_eq!(&new_user.is_admin, &user.is_admin);

        // Update username to "new_admin"
        let rows_updated = diesel::update(users.filter(id.eq(user.id)))
            .set(username.eq("new_admin".to_string()))
            .execute(conn);
        assert_eq!(Ok(1), rows_updated);

        // Read updated_user and check if have changed username
        let updated_user = users.filter(id.eq(user.id)).first::<User>(conn).unwrap();
        assert_eq!(&"new_admin".to_string(), &updated_user.username);
        assert_eq!(&new_user.password, &updated_user.password);
        assert_eq!(&new_user.is_admin, &updated_user.is_admin);

        // Delete user from DB and DB should be empty
        diesel::delete(users.filter(id.eq(user.id))).execute(conn).unwrap();
        assert_user_count(0, conn);
    }
}
