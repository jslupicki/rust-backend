use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::{Crud, HaveId, Searchable};
use crate::contacts_dao::ContactDTO;
use crate::models::{Contact, Employee, NewEmployee, Salary};
use crate::salaries_dao::SalaryDTO;
use crate::schema::contacts::dsl::contacts;
use crate::schema::employees::dsl::id as employee_id;
use crate::schema::employees::dsl::*;
use crate::schema::salaries::dsl::*;

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

fn delete_associations(e_id: i32, conn: &SqliteConnection) -> QueryResult<usize> {
    use crate::schema::contacts::columns::employee_id as contacts_employee_id;
    use crate::schema::salaries::columns::employee_id as salaries_employee_id;

    diesel::delete(salaries)
        .filter(salaries_employee_id.eq(e_id))
        .execute(conn)?;
    diesel::delete(contacts)
        .filter(contacts_employee_id.eq(e_id))
        .execute(conn)
}

impl Crud for EmployeeDTO {
    fn update(&mut self, persisted: &Self) {
        self.id = persisted.id;
        self.salaries = persisted.salaries.clone();
        self.contacts = persisted.contacts.clone();
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Self> {
        employees
            .filter(employee_id.eq(id_to_find))
            .first(conn)
            .map(|e: Employee| into_dto_with_associations(e, conn))
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Self> {
        let employee_to_dto_with_associations = |e: Employee,
                                                 salaries_to_save: &Vec<SalaryDTO>,
                                                 contacts_to_save: &Vec<ContactDTO>|
         -> EmployeeDTO {
            let e_id = e.id;
            let mut e_dto = EmployeeDTO::from(e);
            for s in salaries_to_save {
                let mut new_s = s.clone();
                new_s.employee_id = Some(e_id);
                e_dto.salaries.push(new_s.save_simple(conn).unwrap());
            }
            for c in contacts_to_save {
                let mut new_c = c.clone();
                new_c.employee_id = Some(e_id);
                e_dto.contacts.push(new_c.save_simple(conn).unwrap());
            }
            e_dto
        };
        let insert = |e_dto: &EmployeeDTO| -> QueryResult<EmployeeDTO> {
            insert_into(employees)
                .values(NewEmployee::from(&*e_dto))
                .execute(conn)
                .and_then(|_| {
                    employees
                        .order(employee_id.desc())
                        .first(conn)
                        .map(|e: Employee| {
                            employee_to_dto_with_associations(e, &self.salaries, &self.contacts)
                        })
                })
        };

        if self.id.is_some() {
            let self_id = self.id.unwrap();
            diesel::update(employees.filter(employee_id.eq(self_id)))
                .set(Employee::from(&*self))
                .execute(conn)
                .and_then(|updated| {
                    if updated == 0 {
                        insert(self)
                    } else {
                        employees
                            .filter(employee_id.eq(self_id))
                            .first(conn)
                            .map(|e: Employee| {
                                let _ = delete_associations(e.id, conn);
                                employee_to_dto_with_associations(e, &self.salaries, &self.contacts)
                            })
                    }
                })
        } else {
            insert(self)
        }
    }

    fn delete_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        let _ = delete_associations(id_to_find, conn);
        diesel::delete(employees)
            .filter(employee_id.eq(id_to_find))
            .execute(conn)
    }
}

impl Searchable for EmployeeDTO {
    fn get_all_with_connection(conn: &SqliteConnection) -> Vec<Self> {
        employees
            .load::<Employee>(conn)
            .expect("Load employees failed")
            .into_iter()
            .map(|e| into_dto_with_associations(e, conn))
            .collect()
    }

    fn search_with_connection(s: &str, conn: &SqliteConnection) -> Vec<Self> {
        use crate::schema::employees::columns::search_string;

        employees
            .filter(search_string.like(s))
            .load::<Employee>(conn)
            .expect("Search employees failed ")
            .into_iter()
            .map(Self::from)
            .collect()
    }
}

fn into_dto_with_associations(e: Employee, conn: &SqliteConnection) -> EmployeeDTO {
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
        let common_assertions = |e: &EmployeeDTO, _conn: &SqliteConnection| {
            assert_eq!(e.salaries.len(), 2);
            assert_eq!(e.contacts.len(), 2);
            for s in &e.salaries {
                assert!(s.get_id().is_some());
                assert_eq!(s.employee_id, e.id);
            }
            for c in &e.contacts {
                assert!(c.get_id().is_some());
                assert_eq!(c.employee_id, e.id);
            }
        };
        let assertions = Assertions::new()
            .with_saved(common_assertions)
            .with_get(common_assertions)
            .with_persisted(common_assertions);
        //.with_deleted(|_e, _conn| {});

        employee.test_with_assertion(assertions, conn);
    }
}
