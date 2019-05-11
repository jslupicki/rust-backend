use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;

use models::{Contact, Employee, NewContact, NewEmplyee, NewSalary, Salary};
use schema::contacts::dsl::*;
use schema::employees::dsl::*;
use schema::salaries::dsl::*;

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use diesel;
    use diesel::result::DatabaseErrorKind::UniqueViolation;
    use diesel::result::Error::DatabaseError;
    use log4rs;

    use super::*;

    static TEST_DB_NAME: &str = ":memory:";

    embed_migrations!("./migrations");

    fn initialize_db() -> SqliteConnection {
        let conn = SqliteConnection::establish(TEST_DB_NAME)
            .expect(&format!("Error connecting to {}", TEST_DB_NAME));
        embedded_migrations::run_with_output(&conn, &mut stdout()).unwrap();
        conn
    }
}
