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

impl Crud for EmployeeDTO {
    fn update(&mut self, persisted: &Self) {
        self.id = persisted.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Self> {
        employees
            .filter(employee_id.eq(id_to_find))
            .first(conn)
            .map(|e: Employee| into_dto_with_associations(e, conn))
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Self> {
        let delete_associations = |e_id: i32| {
            use crate::schema::contacts::columns::employee_id as contacts_employee_id;
            use crate::schema::salaries::columns::employee_id as salaries_employee_id;

            // TODO: Consider to delete only required association by use eq_any() in filter
            diesel::delete(salaries)
                .filter(salaries_employee_id.eq(e_id))
                .execute(conn)
                .unwrap();
            diesel::delete(contacts)
                .filter(contacts_employee_id.eq(e_id))
                .execute(conn)
                .unwrap();
        };
        let save_associations = |e_id: i32, e_dto: &mut EmployeeDTO| {
            for s in &mut e_dto.salaries {
                s.employee_id = Some(e_id);
                s.save_simple(conn).map(|saved| s.update(&saved)).unwrap();
            }
            for c in &mut e_dto.contacts {
                c.employee_id = Some(e_id);
                c.save_simple(conn).map(|saved| c.update(&saved)).unwrap();
            }
        };
        let employee_to_dto_with_associations = |e: Employee| -> EmployeeDTO {
            let e_id = e.id;
            let mut e_dto = EmployeeDTO::from(e);
            save_associations(e_id, &mut e_dto);
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
                        .map(|e: Employee| employee_to_dto_with_associations(e))
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
                                delete_associations(e.id);
                                employee_to_dto_with_associations(e)
                            })
                    }
                })
        } else {
            insert(self)
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

impl Searchable for EmployeeDTO {
    fn get_all(conn: &SqliteConnection) -> Vec<Self> {
        employees
            .load::<Employee>(conn)
            .expect("Load employees failed")
            .into_iter()
            .map(|e| into_dto_with_associations(e, conn))
            .collect()
    }

    fn search(s: &str, conn: &SqliteConnection) -> Vec<Self> {
        use crate::schema::employees::columns::search_string;

        employees
            .filter(search_string.like(s))
            .load::<Employee>(conn)
            .expect("Search employees failed ")
            .into_iter()
            .map(|e| Self::from(e))
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
        // TODO: Add assertions about associations
        let assertions = Assertions::new()
            .with_saved(|_e, _conn| {})
            .with_get(|_e, _conn| {})
            .with_persisted(|_e, _conn| {})
            .with_deleted(|_e, _conn| {});
        employee.test_with_assertion(assertions, conn);
    }
}
