use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::SearchableByParent;
use crate::base_dao::{Crud, HaveId};
use crate::models::{NewSalary, Salary};
use crate::schema::salaries::dsl::id as salary_id;
use crate::schema::salaries::dsl::*;
use crate::Searchable;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SalaryDTO {
    pub id: Option<i32>,
    pub employee_id: Option<i32>,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub amount: i64,
    pub search_string: String,
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

impl From<&Salary> for SalaryDTO {
    fn from(s: &Salary) -> Self {
        SalaryDTO {
            id: Some(s.id),
            employee_id: Some(s.employee_id),
            from_date: s.from_date,
            to_date: s.to_date,
            amount: s.amount,
            search_string: s.search_string.clone(),
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

impl HaveId for SalaryDTO {
    fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl Crud for SalaryDTO {
    fn update(&mut self, persisted: &Self) {
        self.id = persisted.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<SalaryDTO> {
        salaries
            .filter(salary_id.eq(id_to_find))
            .first(conn)
            .map(|s: Salary| SalaryDTO::from(s))
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<SalaryDTO> {
        fn insert(s: &SalaryDTO, conn: &SqliteConnection) -> QueryResult<SalaryDTO> {
            insert_into(salaries)
                .values(NewSalary::from(&*s))
                .execute(conn)
                .and_then(|_| {
                    salaries
                        .order(salary_id.desc())
                        .first(conn)
                        .map(|s: Salary| SalaryDTO::from(s))
                })
        }
        if self.id.is_some() {
            let self_id = self.id.unwrap();
            let updated = diesel::update(salaries.filter(salary_id.eq(self_id)))
                .set(Salary::from(&*self))
                .execute(conn)?;
            if updated == 0 {
                insert(self, conn)
            } else {
                salaries
                    .filter(salary_id.eq(self_id))
                    .first(conn)
                    .map(|s: Salary| SalaryDTO::from(s))
            }
        } else {
            insert(self, conn)
        }
    }

    fn delete_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(salaries.filter(salary_id.eq(id_to_find))).execute(conn)
    }
}

impl Searchable for SalaryDTO {
    fn get_all_with_connection(conn: &SqliteConnection) -> Vec<Self> {
        todo!()
    }

    fn search_with_connection(s: &str, conn: &SqliteConnection) -> Vec<Self> {
        todo!()
    }
}

impl SearchableByParent for SalaryDTO {
    fn search_by_parent_id_with_connection(parent_id: i32, conn: &SqliteConnection) -> Vec<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use crate::common_for_tests::*;

    use super::*;

    embed_migrations!("test_data/salaries");

    impl CrudTests for SalaryDTO {}

    #[test]
    fn crud_operations_on_salary() {
        let conn = &initialize();
        embedded_migrations::run_with_output(conn, &mut stdout()).unwrap();
        let mut salary = SalaryDTO {
            id: None,
            employee_id: Some(1),
            from_date: NaiveDate::from_ymd(2015, 3, 14),
            to_date: NaiveDate::from_ymd(2020, 5, 23),
            amount: 0,
            search_string: "some search".to_string(),
        };
        //salary.save_simple(conn).unwrap();
        salary.test(conn);
        //salary.test_without_conn();
    }
}
