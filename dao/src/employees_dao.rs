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

pub fn update_employee(employee: &Employee, conn: &SqliteConnection) -> QueryResult<Employee> {
    conn.transaction(|| {
        diesel::update(employees.filter(employee_id.eq(employee.id)))
            .set(employee)
            .execute(conn)
            .and_then(|_| employees.filter(employee_id.eq(employee.id)).first(conn))
    })
}

pub fn get_employee(id_to_find: i32, conn: &SqliteConnection) -> Option<Employee> {
    employees
        .filter(employee_id.eq(id_to_find))
        .first(conn)
        .optional()
        .unwrap_or(None)
}

pub fn create_salary(new_salary: &NewSalary, conn: &SqliteConnection) -> QueryResult<Salary> {
    conn.transaction(|| {
        insert_into(salaries)
            .values(new_salary)
            .execute(conn)
            .and_then(|_| salaries.order(salary_id.desc()).first(conn))
    })
}

pub fn update_salary(salary: &Salary, conn: &SqliteConnection) -> QueryResult<Salary> {
    conn.transaction(|| {
        diesel::update(salaries.filter(salary_id.eq(salary.id)))
            .set(salary)
            .execute(conn)
            .and_then(|_| salaries.filter(salary_id.eq(salary.id)).first(conn))
    })
}

pub fn get_salary(id_to_find: i32, conn: &SqliteConnection) -> Option<Salary> {
    salaries
        .filter(salary_id.eq(id_to_find))
        .first(conn)
        .optional()
        .unwrap_or(None)
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

    #[test]
    fn check_update_employee() {
        let conn = &initialize();

        let new_employee = NewEmplyee {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            search_string: "some search string".to_string(),
        };
        let created_employee = create_employee(&new_employee, conn).unwrap();

        let employee_to_update = Employee {
            id: created_employee.id,
            first_name: created_employee.first_name,
            last_name: created_employee.last_name,
            search_string: "different search string".to_string(),
        };

        update_employee(&employee_to_update, conn);

        let updated_employee = get_employee(employee_to_update.id, conn).unwrap();
        assert_eq!(updated_employee.id, employee_to_update.id);
        assert_eq!(updated_employee.first_name, employee_to_update.first_name);
        assert_eq!(updated_employee.last_name, employee_to_update.last_name);
        assert_eq!(
            updated_employee.search_string,
            employee_to_update.search_string
        );
        assert_ne!(updated_employee.search_string, new_employee.search_string);
    }
}
