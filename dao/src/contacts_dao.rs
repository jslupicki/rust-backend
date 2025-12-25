use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::SearchableByParent;
use crate::base_dao::{Crud, HaveId};
use crate::models::{Contact, NewContact};
use crate::schema::contacts::dsl::id as contact_id;
use crate::schema::contacts::dsl::*;
use crate::Searchable;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContactDTO {
    pub id: Option<i32>,
    pub employee_id: Option<i32>,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub phone: String,
    pub address: Option<String>,
    pub search_string: String,
}

impl From<Contact> for ContactDTO {
    fn from(c: Contact) -> Self {
        ContactDTO {
            id: Some(c.id),
            employee_id: Some(c.employee_id),
            from_date: c.from_date,
            to_date: c.to_date,
            address: c.address,
            phone: c.phone,
            search_string: c.search_string,
        }
    }
}

impl From<&Contact> for ContactDTO {
    fn from(c: &Contact) -> Self {
        ContactDTO {
            id: Some(c.id),
            employee_id: Some(c.employee_id),
            from_date: c.from_date,
            to_date: c.to_date,
            address: c.address.clone(),
            phone: c.phone.clone(),
            search_string: c.search_string.clone(),
        }
    }
}

impl From<&ContactDTO> for Contact {
    fn from(contact_dto: &ContactDTO) -> Self {
        Contact {
            id: contact_dto.id.unwrap(),
            employee_id: contact_dto.employee_id.unwrap(),
            from_date: contact_dto.from_date,
            to_date: contact_dto.to_date,
            address: contact_dto.address.clone(),
            phone: contact_dto.phone.clone(),
            search_string: contact_dto.search_string.clone(),
        }
    }
}

impl From<&ContactDTO> for NewContact {
    fn from(contact_dto: &ContactDTO) -> Self {
        NewContact {
            employee_id: contact_dto.employee_id.unwrap(),
            from_date: contact_dto.from_date,
            to_date: contact_dto.to_date,
            address: contact_dto.address.clone(),
            phone: contact_dto.phone.clone(),
            search_string: contact_dto.search_string.clone(),
        }
    }
}

impl HaveId for ContactDTO {
    fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl Crud for ContactDTO {
    fn update(&mut self, persisted: &Self) {
        self.id = persisted.id;
    }

    fn get_simple(id_to_find: i32, conn: &mut SqliteConnection) -> QueryResult<ContactDTO> {
        contacts
            .filter(contact_id.eq(id_to_find))
            .first(conn)
            .map(|c: Contact| ContactDTO::from(c))
    }

    fn save_simple(&self, conn: &mut SqliteConnection) -> QueryResult<ContactDTO> {
        fn insert(c: &ContactDTO, conn: &mut SqliteConnection) -> QueryResult<ContactDTO> {
            insert_into(contacts)
                .values(NewContact::from(c))
                .execute(conn)
                .and_then(|_| {
                    contacts
                        .order(contact_id.desc())
                        .first(conn)
                        .map(|c: Contact| ContactDTO::from(c))
                })
        }
        if self.id.is_some() {
            let self_id = self.id.unwrap();
            let updated = diesel::update(contacts.filter(contact_id.eq(self_id)))
                .set(Contact::from(self))
                .execute(conn)?;
            if updated == 0 {
                insert(self, conn)
            } else {
                contacts
                    .filter(contact_id.eq(self_id))
                    .first(conn)
                    .map(|c: Contact| ContactDTO::from(c))
            }
        } else {
            insert(self, conn)
        }
    }

    fn delete_simple(id_to_find: i32, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::delete(contacts.filter(contact_id.eq(id_to_find))).execute(conn)
    }
}

impl Searchable for ContactDTO {
    fn get_all_with_connection(conn: &mut SqliteConnection) -> Vec<Self> {
        todo!()
    }

    fn search_with_connection(s: &str, conn: &mut SqliteConnection) -> Vec<Self> {
        todo!()
    }
}

impl SearchableByParent for ContactDTO {
    fn search_by_parent_id_with_connection(parent_id: i32, conn: &mut SqliteConnection) -> Vec<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

    use crate::common_for_tests::*;

    use super::*;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("test_data/contacts");

    impl CrudTests for ContactDTO {}

    #[test]
    fn crud_operations_on_contacts() {
        let conn = &mut initialize();
        conn.run_pending_migrations(MIGRATIONS);
        let mut contact = ContactDTO {
            id: None,
            employee_id: Some(1),
            from_date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
            to_date: NaiveDate::from_ymd_opt(2020, 5, 23).unwrap(),
            phone: "123456".to_string(),
            address: Some("Some contact address".to_string()),
            search_string: "some search for contact".to_string(),
        };
        //salary.save_simple(conn).unwrap();
        contact.test(conn);
        //salary.test_without_conn();
    }
}
