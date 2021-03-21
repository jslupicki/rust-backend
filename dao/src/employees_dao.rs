use std::collections::HashSet;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::{Crud, HaveId};
use crate::contacts_dao::ContactDTO;
use crate::models::{Employee, NewEmployee, NewSalary, Salary};
use crate::salaries_dao::SalaryDTO;
use crate::schema::contacts::dsl::contacts;
use crate::schema::employees::dsl::id as employee_id;
use crate::schema::employees::dsl::*;
use crate::schema::salaries::dsl::id as salary_id;
use crate::schema::salaries::dsl::*;
use crate::Contact;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeDTO {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub search_string: String,
    pub salaries: Vec<SalaryDTO>,
    pub contacts: Vec<ContactDTO>,
}

impl From<Employee> for EmployeeDTO {
    fn from(e: Employee) -> Self {
        EmployeeDTO {
            id: Some(e.id),
            first_name: e.first_name,
            last_name: e.last_name,
            search_string: e.search_string,
            salaries: Default::default(),
            contacts: Default::default(),
        }
    }
}

impl From<&EmployeeDTO> for Employee {
    fn from(employee_dto: &EmployeeDTO) -> Self {
        Employee {
            id: employee_dto.id.unwrap(),
            first_name: employee_dto.first_name.clone(),
            last_name: employee_dto.last_name.clone(),
            search_string: employee_dto.search_string.clone(),
        }
    }
}

impl From<&EmployeeDTO> for NewEmployee {
    fn from(employee_dto: &EmployeeDTO) -> Self {
        NewEmployee {
            first_name: employee_dto.first_name.clone(),
            last_name: employee_dto.last_name.clone(),
            search_string: employee_dto.search_string.clone(),
        }
    }
}

impl HaveId for EmployeeDTO {
    fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl Crud for EmployeeDTO {
    fn update(&mut self, other: &Self) {
        self.id = other.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Self> {
        employees
            .filter(employee_id.eq(id_to_find))
            .first(conn)
            .map(|e: Employee| {
                let sv: Vec<Salary> = Salary::belonging_to(&e).load(conn).unwrap();
                let cv: Vec<Contact> = Contact::belonging_to(&e).load(conn).unwrap();
                let mut e_dto = EmployeeDTO::from(e);
                for s in sv {
                    e_dto.salaries.push(SalaryDTO::from(s));
                }
                for c in cv {
                    e_dto.contacts.push(ContactDTO::from(c));
                }
                e_dto
            })
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Self> {
        fn insert(e: &EmployeeDTO, conn: &SqliteConnection) -> QueryResult<EmployeeDTO> {
            insert_into(employees)
                .values(NewEmployee::from(&*e))
                .execute(conn)
                .and_then(|_| {
                    employees
                        .order(employee_id.desc())
                        .first(conn)
                        .map(|e: Employee| EmployeeDTO::from(e))
                })
        }

        let result = if self.id.is_some() {
            let self_id = self.id.unwrap();
            let updated = diesel::update(employees.filter(employee_id.eq(self_id)))
                .set(Employee::from(&*self))
                .execute(conn)?;
            if updated == 0 {
                insert(self, conn)
            } else {
                employees
                    .filter(employee_id.eq(self_id))
                    .first(conn)
                    .map(|e: Employee| EmployeeDTO::from(e))
            }
        } else {
            insert(self, conn)
        };
        match result {
            Ok(mut e_dto) => {
                for s in &self.salaries {
                    let mut new_s = s.clone();
                    new_s.employee_id = e_dto.id;
                    e_dto.salaries.push(new_s.save_simple(conn)?);
                }
                for c in &self.contacts {
                    let mut new_c = c.clone();
                    new_c.employee_id = e_dto.id;
                    e_dto.contacts.push(new_c.save_simple(conn)?);
                }
                Ok(e_dto)
            }
            Err(_) => result,
        }
    }

    fn delete_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        use crate::schema::contacts::columns::employee_id as contacts_employee_id;
        use crate::schema::salaries::columns::employee_id as salaries_employee_id;

        diesel::delete(salaries)
            .filter(salaries_employee_id.eq(id_to_find))
            .execute(conn)?;
        diesel::delete(contacts)
            .filter(contacts_employee_id.eq(id_to_find))
            .execute(conn)?;
        diesel::delete(employees)
            .filter(employee_id.eq(id_to_find))
            .execute(conn)
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
    use chrono::NaiveDate;

    use crate::base_dao::Crud;
    use crate::common_for_tests::*;
    use crate::schema::contacts::dsl::contacts;
    use crate::{Contact, NewContact};

    use super::*;

    #[test]
    fn check_get_simple() {
        let conn = &initialize();

        let new_employee: Employee = insert_into(employees)
            .values(NewEmployee {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
                search_string: "".to_string(),
            })
            .execute(conn)
            .and_then(|_| employees.order(employee_id.desc()).first(conn))
            .unwrap();
        let new_contacts = vec![
            NewContact {
                employee_id: new_employee.id,
                from_date: NaiveDate::from_ymd(2015, 3, 14),
                to_date: NaiveDate::from_ymd(2015, 3, 15),
                phone: "123456".to_string(),
                address: None,
                search_string: "".to_string(),
            },
            NewContact {
                employee_id: new_employee.id,
                from_date: NaiveDate::from_ymd(2015, 3, 16),
                to_date: NaiveDate::from_ymd(2015, 3, 17),
                phone: "234567".to_string(),
                address: None,
                search_string: "".to_string(),
            },
        ];
        insert_into(contacts)
            .values(&new_contacts)
            .execute(conn)
            .unwrap();
        let new_salaries = vec![
            NewSalary {
                employee_id: new_employee.id,
                from_date: NaiveDate::from_ymd(2015, 3, 14),
                to_date: NaiveDate::from_ymd(2015, 3, 15),
                amount: 1,
                search_string: "".to_string(),
            },
            NewSalary {
                employee_id: new_employee.id,
                from_date: NaiveDate::from_ymd(2015, 3, 16),
                to_date: NaiveDate::from_ymd(2015, 3, 17),
                amount: 2,
                search_string: "".to_string(),
            },
        ];
        insert_into(salaries)
            .values(&new_salaries)
            .execute(conn)
            .unwrap();

        let results: Vec<(Employee, Contact, Salary)> = employees
            .inner_join(contacts)
            .inner_join(salaries)
            .filter(employee_id.eq(new_employee.id))
            .load(conn)
            .unwrap();

        for (employee, contact, salary) in results {
            println!("e: {:?} -> c: {:?}, s: {:?}", employee, contact, salary);
        }

        let salaries_of_empleyee: Vec<Salary> =
            Salary::belonging_to(&new_employee).load(conn).unwrap();
        for salary in salaries_of_empleyee {
            println!("Salary: {:?}", salary);
        }
        let contacts_of_employee: Vec<Contact> =
            Contact::belonging_to(&new_employee).load(conn).unwrap();
        for contact in contacts_of_employee {
            println!("Contact: {:?}", contact);
        }

        let e_dto = EmployeeDTO::get_with_conn(new_employee.id, conn);
        println!("e_dto: {:?}", e_dto);

        let count = e_dto.unwrap().delete_with_conn(conn);
        println!("Removed employee: {:?}", count);

        let salaries_in_db: Vec<Salary> = salaries.load(conn).unwrap();
        let contacts_in_db: Vec<Contact> = contacts.load(conn).unwrap();

        println!("In DB left salaries: {:?}", salaries_in_db);
        println!("In DB left contacts: {:?}", contacts_in_db);

        let new_e_dto = EmployeeDTO {
            id: None,
            first_name: "Bartlomiej".to_string(),
            last_name: "Nowak".to_string(),
            search_string: "".to_string(),
            salaries: vec![
                SalaryDTO {
                    id: None,
                    employee_id: None,
                    from_date: NaiveDate::from_ymd(2015, 3, 14),
                    to_date: NaiveDate::from_ymd(2015, 3, 15),
                    amount: 1,
                    search_string: "".to_string(),
                },
                SalaryDTO {
                    id: None,
                    employee_id: None,
                    from_date: NaiveDate::from_ymd(2015, 3, 16),
                    to_date: NaiveDate::from_ymd(2015, 3, 17),
                    amount: 2,
                    search_string: "".to_string(),
                },
            ],
            contacts: vec![
                ContactDTO {
                    id: None,
                    employee_id: None,
                    from_date: NaiveDate::from_ymd(2015, 3, 14),
                    to_date: NaiveDate::from_ymd(2015, 3, 15),
                    phone: "123456".to_string(),
                    address: Some("Address 1".to_string()),
                    search_string: "".to_string(),
                },
                ContactDTO {
                    id: None,
                    employee_id: None,
                    from_date: NaiveDate::from_ymd(2015, 3, 16),
                    to_date: NaiveDate::from_ymd(2015, 3, 17),
                    phone: "234567".to_string(),
                    address: Some("Address 2".to_string()),
                    search_string: "".to_string(),
                },
            ],
        };
        let saved_new_e_dto = new_e_dto.save_simple(conn);

        println!("new_e_dto: {:?}", new_e_dto);
        println!("saved_new_e_dto: {:?}", saved_new_e_dto);
    }

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

        update_employee(&employee_to_update, conn).unwrap();

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

        new_salary.persist_in_transaction(conn);

        assert_eq!(Some(1), new_salary.id);
        assert_eq!(123, new_salary.amount);

        new_salary.amount = 124;
        new_salary.persist_in_transaction(conn);

        let salary = SalaryDTO::get_with_conn(new_salary.id.unwrap(), conn);

        assert!(salary.is_some());
        assert_eq!(124, salary.unwrap().amount);
    }

    #[test]
    fn check_create_contact() {
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
    fn check_persist_contact() {
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

        new_salary.persist_in_transaction(conn);

        assert_eq!(Some(1), new_salary.id);
        assert_eq!(123, new_salary.amount);

        new_salary.amount = 124;
        new_salary.persist_in_transaction(conn);

        let salary = SalaryDTO::get_with_conn(new_salary.id.unwrap(), conn);

        assert!(salary.is_some());
        assert_eq!(124, salary.unwrap().amount);
    }
}
