use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;

use models::{Contact, Employee, NewContact, NewEmplyee, NewSalary, Salary};
use schema::contacts::dsl::id as contact_id;
use schema::contacts::dsl::*;
use schema::employees::dsl::id as employee_id;
use schema::employees::dsl::*;
use schema::salaries::dsl::id as salary_id;
use schema::salaries::dsl::*;

pub fn create_employee(
    new_employee: &NewEmplyee,
    conn: &SqliteConnection,
) -> QueryResult<Employee> {
    conn.transaction(|| {
        insert_into(employees)
            .values(new_employee)
            .execute(conn)
            .and_then(|_| employees.order(employee_id.desc()).first(conn))
    })
}

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use diesel;
    use diesel::result::DatabaseErrorKind::UniqueViolation;
    use diesel::result::Error::DatabaseError;
    use log4rs;

    use common_for_tests::*;

    use super::*;

    #[test]
    fn check_create_employee() {
        let conn = &initialize();

        let new_employee = NewEmplyee {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            search_string: "some search string".to_string(),
        };

        assert_employee_count(0, conn);

        let created_employee = create_employee(&new_employee, conn).unwrap();

        assert_employee_count(1, conn);
        assert_eq!(1, created_employee.id);
    }
}
