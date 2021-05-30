use chrono::NaiveDate;

use crate::schema::{contacts, employees, salaries, users};

#[derive(Queryable, AsChangeset, Debug, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Queryable, AsChangeset, Debug, Serialize, Identifiable, Clone)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub search_string: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "employees"]
pub struct NewEmployee {
    pub first_name: String,
    pub last_name: String,
    pub search_string: String,
}

#[derive(Queryable, AsChangeset, Debug, Serialize, Associations, Identifiable, Clone)]
#[belongs_to(Employee)]
#[table_name = "salaries"]
pub struct Salary {
    pub id: i32,
    pub employee_id: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub amount: i64,
    pub search_string: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "salaries"]
pub struct NewSalary {
    pub employee_id: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub amount: i64,
    pub search_string: String,
}

#[derive(Queryable, AsChangeset, Debug, Serialize, Associations, Identifiable, Clone)]
#[belongs_to(Employee)]
#[table_name = "contacts"]
pub struct Contact {
    pub id: i32,
    pub employee_id: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub phone: String,
    pub address: Option<String>,
    pub search_string: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "contacts"]
pub struct NewContact {
    pub employee_id: i32,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub phone: String,
    pub address: Option<String>,
    pub search_string: String,
}
