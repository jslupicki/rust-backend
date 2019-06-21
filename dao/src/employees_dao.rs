use std::collections::HashSet;

use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use connection::POOL;
use models::{Contact, Employee, NewContact, NewEmployee, NewSalary, Salary};
use schema::contacts::dsl::id as contact_id;
use schema::contacts::dsl::*;
use schema::employees::dsl::id as employee_id;
use schema::employees::dsl::*;
use schema::salaries::dsl::id as salary_id;
use schema::salaries::dsl::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
struct SalaryDTO {
    id: Option<i32>,
    employee_id: Option<i32>,
    from_date: NaiveDate,
    to_date: NaiveDate,
    amount: i64,
    search_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
struct ContactDTO {
    id: Option<i32>,
    employee_id: Option<i32>,
    from_date: NaiveDate,
    to_date: NaiveDate,
    phone: String,
    address: Option<String>,
    search_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EmployeeDTO {
    id: Option<i32>,
    first_name: String,
    last_name: String,
    search_string: String,
    salaries: HashSet<SalaryDTO>,
    contacts: HashSet<ContactDTO>,
}

trait Crud {
    fn get(id: i32) -> QueryResult<Self>
    where
        Self: Sized;
    fn persist(&mut self) -> Option<&Self>;
}

impl From<Salary> for SalaryDTO {
    fn from(s: Salary) -> Self {
        SalaryDTO {
            id: Some(s.id),
            employee_id: Some(s.employee_id),
            from_date: s.from_date,
            to_date: s.to_date,
            amount: s.amount,
            search_string: s.search_string,
        }
    }
}

impl From<&SalaryDTO> for Salary {
    fn from(salary_dto: &SalaryDTO) -> Self {
        Salary {
            id: salary_dto.id.unwrap(),
            employee_id: salary_dto.employee_id.unwrap(),
            from_date: salary_dto.from_date,
            to_date: salary_dto.to_date,
            amount: salary_dto.amount,
            search_string: salary_dto.search_string.clone(),
        }
    }
}

impl From<&SalaryDTO> for NewSalary {
    fn from(salary_dto: &SalaryDTO) -> Self {
        NewSalary {
            employee_id: salary_dto.employee_id.unwrap(),
            from_date: salary_dto.from_date,
            to_date: salary_dto.to_date,
            amount: salary_dto.amount,
            search_string: salary_dto.search_string.clone(),
        }
    }
}

impl SalaryDTO {
    fn get_with_conn(id_to_find: i32, conn: &SqliteConnection) -> Option<Self> {
        salaries
            .filter(salary_id.eq(id_to_find))
            .first(conn)
            .optional()
            .unwrap_or(None)
            .map(|s: Salary| SalaryDTO::from(s))
    }

    fn get(id_to_find: i32) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        Self::get_with_conn(id_to_find, conn)
    }

    fn persist_with_conn(&mut self, conn: &SqliteConnection) -> Option<Self> {
        conn.transaction(|| {
            if self.id.is_some() {
                let self_id = self.id.unwrap();
                diesel::update(salaries.filter(salary_id.eq(self_id)))
                    .set(Salary::from(&*self))
                    .execute(conn)
                    .and_then(|_| salaries.filter(salary_id.eq(self_id)).first(conn))
            } else {
                insert_into(salaries)
                    .values(NewSalary::from(&*self))
                    .execute(conn)
                    .and_then(|_| salaries.order(salary_id.desc()).first(conn))
            }
        })
        .optional()
        .unwrap_or(None)
        .map(|s: Salary| {
            self.id = Some(s.id);
            self.employee_id = Some(s.employee_id);
            self.from_date = s.from_date;
            self.to_date = s.to_date;
            self.amount = s.amount;
            self.search_string = s.search_string.clone();
            s.into()
        })
    }

    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        self.persist_with_conn(conn)
    }
}

pub fn create_employee(
    new_employee: &NewEmployee,
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
    use common_for_tests::*;

    use super::*;

    #[test]
    fn check_create_employee() {
        let conn = &initialize();

        let new_employee = NewEmployee {
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

        let new_employee = NewEmployee {
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

    #[test]
    fn check_create_salary() {
        let conn = &initialize();

        let created_employee = create_employee(
            &NewEmployee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                search_string: "some search string".to_string(),
            },
            conn,
        )
        .unwrap();

        let new_salary = NewSalary {
            employee_id: created_employee.id,
            amount: 123,
            from_date: NaiveDate::from_ymd(2019, 6, 18),
            to_date: NaiveDate::from_ymd(2019, 6, 19),
            search_string: "some search string".to_string(),
        };

        let created_salary = create_salary(&new_salary, conn).unwrap();
        assert_eq!(1, created_salary.id);

        let salary_dto = SalaryDTO::get_with_conn(created_salary.id, conn);
        assert!(salary_dto.is_some());
        let salary_dto = salary_dto.unwrap();
        assert_eq!(Some(created_salary.id), salary_dto.id);
        assert_eq!(Some(created_employee.id), salary_dto.employee_id);
        assert_eq!(created_salary.from_date, salary_dto.from_date);
        assert_eq!(created_salary.to_date, salary_dto.to_date);
        assert_eq!(created_salary.amount, salary_dto.amount);
        assert_eq!(created_salary.search_string, salary_dto.search_string);
        let salary_dto = SalaryDTO::get_with_conn(123, conn);
        assert!(salary_dto.is_none());
    }

    #[test]
    fn check_persist_salary() {
        let conn = &initialize();

        let created_employee = create_employee(
            &NewEmployee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                search_string: "some search string".to_string(),
            },
            conn,
        )
        .unwrap();

        let mut new_salary = SalaryDTO {
            id: None,
            employee_id: Some(created_employee.id),
            amount: 123,
            from_date: NaiveDate::from_ymd(2019, 6, 18),
            to_date: NaiveDate::from_ymd(2019, 6, 19),
            search_string: "some search string".to_string(),
        };

        new_salary.persist_with_conn(conn);

        assert_eq!(Some(1), new_salary.id);
        assert_eq!(123, new_salary.amount);

        new_salary.amount = 124;
        new_salary.persist_with_conn(conn);

        let salary = SalaryDTO::get_with_conn(new_salary.id.unwrap(), conn);

        assert!(salary.is_some());
        assert_eq!(124, salary.unwrap().amount);
    }
}
