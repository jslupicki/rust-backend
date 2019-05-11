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

    use common_for_tests::*;

    use super::*;
}
