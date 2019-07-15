use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use base_dao::Crud;
use connection::POOL;
use models::{NewSalary, Salary};
use schema::salaries::dsl::id as salary_id;
use schema::salaries::dsl::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SalaryDTO {
    id: Option<i32>,
    employee_id: Option<i32>,
    from_date: NaiveDate,
    to_date: NaiveDate,
    amount: i64,
    search_string: String,
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

    fn save_with_conn(&self, conn: &SqliteConnection) -> Option<Self> {
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
        .map(|s: Salary| s.into())
    }

    fn save(&self) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        self.save_with_conn(conn)
    }

    fn persist_with_conn(&mut self, conn: &SqliteConnection) -> Option<Self> {
        self.save_with_conn(conn).map(|s| {
            self.id = s.id;
            s
        })
    }

    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        self.persist_with_conn(conn)
    }
}

impl Crud<Salary> for SalaryDTO {
    fn update(&mut self, other: &Self) {
        self.id = other.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Salary> {
        salaries.filter(salary_id.eq(id_to_find)).first(conn)
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Salary> {
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
    }
}
