use std::io::stdout;

#[cfg(test)]
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use schema::employees::dsl::id as employee_id;
use schema::employees::dsl::*;
use schema::users::dsl::id as user_id;
use schema::users::dsl::*;

static TEST_DB_NAME: &str = ":memory:";

embed_migrations!("./migrations");

pub fn initialize_log() {
    let _ = log4rs::init_file("log4rs.yml", Default::default());
}

pub fn initialize_db() -> SqliteConnection {
    let conn = SqliteConnection::establish(TEST_DB_NAME)
        .expect(&format!("Error connecting to {}", TEST_DB_NAME));
    embedded_migrations::run_with_output(&conn, &mut stdout()).unwrap();
    conn
}

pub fn initialize() -> SqliteConnection {
    initialize_log();
    initialize_db()
}

pub fn user_count(conn: &SqliteConnection) -> i64 {
    users.select(count(user_id)).first(conn).unwrap()
}

pub fn employee_count(conn: &SqliteConnection) -> i64 {
    employees.select(count(employee_id)).first(conn).unwrap()
}

pub fn assert_user_count(expected: i64, conn: &SqliteConnection) {
    let user_count = user_count(conn);
    assert_eq!(user_count, expected);
}

pub fn assert_employee_count(expected: i64, conn: &SqliteConnection) {
    let employee_count = employee_count(conn);
    assert_eq!(employee_count, expected);
}
