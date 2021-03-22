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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::common_for_tests::*;

    use super::*;

    impl CrudTests for EmployeeDTO {}

    #[test]
    fn crud_operations_on_employee() {
        let conn = &initialize();
        let mut employee = EmployeeDTO {
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
        employee.test(conn);
    }
}
