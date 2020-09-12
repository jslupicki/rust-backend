use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::{Crud, HaveId};
use crate::connection::get_connection;
use crate::models::{Contact, NewContact};
use crate::schema::contacts::dsl::id as contact_id;
use crate::schema::contacts::dsl::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContactDTO {
    id: Option<i32>,
    employee_id: Option<i32>,
    from_date: NaiveDate,
    to_date: NaiveDate,
    phone: String,
    address: Option<String>,
    search_string: String,
}

impl From<Contact> for ContactDTO {
    fn from(c: Contact) -> Self {
        ContactDTO {
            id: Some(c.id),
            employee_id: Some(c.id),
            from_date: c.from_date,
            to_date: c.to_date,
            address: c.address,
            phone: c.phone,
            search_string: c.search_string,
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

    fn update(&mut self, other: &Self) {
        self.id = other.id;
    }

    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<ContactDTO> {
        contacts.filter(contact_id.eq(id_to_find)).first(conn).map(|c: Contact| ContactDTO::from(c))
    }

    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<ContactDTO> {
        fn insert(c: &ContactDTO, conn: &SqliteConnection) -> QueryResult<ContactDTO> {
            insert_into(contacts)
                .values(NewContact::from(&*c))
                .execute(conn)
                .and_then(|_| contacts.order(contact_id.desc()).first(conn).map(|c: Contact| ContactDTO::from(c)))
        }
        if self.id.is_some() {
            let self_id = self.id.unwrap();
            let updated = diesel::update(contacts.filter(contact_id.eq(self_id)))
                .set(Contact::from(&*self))
                .execute(conn)?;
            if updated == 0 {
                insert(self, conn)
            } else {
                contacts.filter(contact_id.eq(self_id)).first(conn).map(|c: Contact| ContactDTO::from(c))
            }
        } else {
            insert(self, conn)
        }
    }

    fn delete_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(contacts.filter(contact_id.eq(id_to_find))).execute(conn)
    }
}
