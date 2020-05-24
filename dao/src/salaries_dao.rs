use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use base_dao::Crud;
use models::{NewSalary, Salary};
use schema::salaries::dsl::id as salary_id;
use schema::salaries::dsl::*;

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

impl Crud for SalaryDTO {
    type DbType = Salary;

    fn get_id(&self) -> Option<i32> {
        self.id
    }

    fn update(&mut self, other: &Self) {
        self.id = other.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Salary> {
        salaries.filter(salary_id.eq(id_to_find)).first(conn)
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Salary> {
        fn insert(s: &SalaryDTO, conn: &SqliteConnection) -> QueryResult<Salary> {
            insert_into(salaries)
                .values(NewSalary::from(&*s))
                .execute(conn)
                .and_then(|_| salaries.order(salary_id.desc()).first(conn))
        }
        if self.id.is_some() {
            let self_id = self.id.unwrap();
            let updated = diesel::update(salaries.filter(salary_id.eq(self_id)))
                .set(Salary::from(&*self))
                .execute(conn)?;
            if updated == 0 {
                insert(self, conn)
            } else {
                salaries.filter(salary_id.eq(self_id)).first(conn)
            }
        } else {
            insert(self, conn)
        }
    }

    fn delete_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(salaries.filter(salary_id.eq(id_to_find))).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use common_for_tests::*;

    use super::*;

    impl CrudTests for SalaryDTO {}

    #[test]
    fn crud_operations_on_salary() {
        let conn = &initialize();
        let mut salary = SalaryDTO {
            id: None,
            employee_id: Some(12),
            from_date: NaiveDate::from_ymd(2015, 3, 14),
            to_date: NaiveDate::from_ymd(2020, 5, 23),
            amount: 0,
            search_string: "some search".to_string(),
        };
        salary.test(conn);
        //salary.test_without_conn();
    }
}
