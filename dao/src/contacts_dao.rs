use chrono::NaiveDate;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use connection::get_connection;
use models::{Contact, NewContact};
use schema::contacts::dsl::id as contact_id;
use schema::contacts::dsl::*;

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

impl ContactDTO {
    fn get_with_conn(id_to_find: i32, conn: &SqliteConnection) -> Option<Self> {
        contacts
            .filter(contact_id.eq(id_to_find))
            .first(conn)
            .optional()
            .unwrap_or(None)
            .map(|c: Contact| ContactDTO::from(c))
    }

    fn get(id_to_find: i32) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        Self::get_with_conn(id_to_find, conn)
    }

    fn save_with_conn(&self, conn: &SqliteConnection) -> Option<Self> {
        conn.transaction(|| {
            if self.id.is_some() {
                let self_id = self.id.unwrap();
                diesel::update(contacts.filter(contact_id.eq(self_id)))
                    .set(Contact::from(&*self))
                    .execute(conn)
                    .and_then(|_| contacts.filter(contact_id.eq(self_id)).first(conn))
            } else {
                insert_into(contacts)
                    .values(NewContact::from(&*self))
                    .execute(conn)
                    .and_then(|_| contacts.order(contact_id.desc()).first(conn))
            }
        })
        .optional()
        .unwrap_or(None)
        .map(|c: Contact| c.into())
    }

    fn save(&self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.save_with_conn(conn)
    }

    fn persist_with_conn(&mut self, conn: &SqliteConnection) -> Option<Self> {
        self.save_with_conn(conn).map(|c| {
            self.id = c.id;
            c
        })
    }

    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.persist_with_conn(conn)
    }
}
