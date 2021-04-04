use std::fmt::Debug;
use std::io::stdout;

#[cfg(test)]
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::base_dao::{Crud, HaveId};
use crate::schema::employees::dsl::id as employee_id;
use crate::schema::employees::dsl::*;
use crate::schema::users::dsl::id as user_id;
use crate::schema::users::dsl::*;

static TEST_DB_NAME: &str = ":memory:";

embed_migrations!("../migrations");

pub fn initialize_log() {
    let _ = log4rs::init_file("log4rs.yml", Default::default());
}

pub fn initialize_db() -> SqliteConnection {
    let conn = SqliteConnection::establish(TEST_DB_NAME)
        .expect(&format!("Error connecting to {}", TEST_DB_NAME));
    embedded_migrations::run_with_output(&conn, &mut stdout()).unwrap();
    conn
}

pub fn initialize() -> SqliteConnection {
    initialize_log();
    initialize_db()
}

pub fn user_count(conn: &SqliteConnection) -> i64 {
    users.select(count(user_id)).first(conn).unwrap()
}

pub fn employee_count(conn: &SqliteConnection) -> i64 {
    employees.select(count(employee_id)).first(conn).unwrap()
}

pub fn assert_user_count(expected: i64, conn: &SqliteConnection) {
    let user_count = user_count(conn);
    assert_eq!(user_count, expected);
}

pub fn assert_employee_count(expected: i64, conn: &SqliteConnection) {
    let employee_count = employee_count(conn);
    assert_eq!(employee_count, expected);
}

pub struct Assertions<T> {
    pub saved: Option<fn(&T, &SqliteConnection)>,
    pub get: Option<fn(&T, &SqliteConnection)>,
    pub persisted: Option<fn(&T, &SqliteConnection)>,
    pub deleted: Option<fn(&T, &SqliteConnection)>,
}

impl<T> Assertions<T> {
    pub fn new() -> Assertions<T> {
        Assertions {
            saved: None,
            get: None,
            persisted: None,
            deleted: None,
        }
    }
    pub fn with_saved(mut self, f: fn(&T, &SqliteConnection)) -> Self {
        self.saved = Some(f);
        self
    }
    pub fn with_get(mut self, f: fn(&T, &SqliteConnection)) -> Self {
        self.get = Some(f);
        self
    }
    pub fn with_persisted(mut self, f: fn(&T, &SqliteConnection)) -> Self {
        self.persisted = Some(f);
        self
    }
    pub fn with_deleted(mut self, f: fn(&T, &SqliteConnection)) -> Self {
        self.deleted = Some(f);
        self
    }
}

pub trait CrudTests
where
    Self: Crud + HaveId + Debug,
{
    fn test(&mut self, conn: &SqliteConnection) {
        Self::test_with_assertion(self, Assertions::new(), conn);
    }

    fn test_with_assertion(&mut self, assertions: Assertions<Self>, conn: &SqliteConnection) {
        info!("About to test {:#?}", &self);
        // Save
        let saved = self.save_in_transaction(conn);
        assert!(saved.is_some());
        let saved_id = saved.unwrap().get_id();
        assert!(saved_id.is_some());
        let saved_id = saved_id.unwrap();
        // Get
        let saved = Self::get_with_conn(saved_id, conn);
        assert!(saved.is_some());
        let saved_id2 = saved.unwrap().get_id();
        assert!(saved_id2.is_some());
        let saved_id2 = saved_id2.unwrap();
        assert_eq!(saved_id, saved_id2);
        if let Some(f) = assertions.get {
            f(self, conn);
        }
        // Persist
        assert!(self.get_id().is_none());
        let persisted = self.persist_in_transaction(conn);
        assert!(persisted.is_some());
        assert!(self.get_id().is_some());
        let persisted_id = persisted.unwrap().get_id();
        assert!(persisted_id.is_some());
        let persisted_id = persisted_id.unwrap();
        let self_id = self.get_id().unwrap();
        assert_eq!(self_id, persisted_id);
        // Delete by id
        let deleted = self.delete_with_conn(conn);
        assert_eq!(deleted, Some(1));
        let just_deleted = Self::get_with_conn(self_id, conn);
        assert!(just_deleted.is_none());
        // Delete by self
        self.persist_in_transaction(conn);
        let self_id = self.get_id().unwrap();
        let persisted = Self::get_with_conn(self_id, conn);
        assert!(persisted.is_some());
        let deleted = self.delete_with_conn(conn);
        assert_eq!(deleted, Some(1));
        let just_deleted = Self::get_with_conn(self_id, conn);
        assert!(just_deleted.is_none());
    }

    fn test_without_conn(&mut self) {
        info!("About to test {:#?}", &self);
        // Save
        let saved = self.save();
        assert!(saved.is_some());
        let saved_id = saved.unwrap().get_id();
        assert!(saved_id.is_some());
        let saved_id = saved_id.unwrap();
        // Get
        let saved = Self::get(saved_id);
        assert!(saved.is_some());
        let saved_id2 = saved.unwrap().get_id();
        assert!(saved_id2.is_some());
        let saved_id2 = saved_id2.unwrap();
        assert_eq!(saved_id, saved_id2);
        // Persist
        assert!(self.get_id().is_none());
        let persisted = self.persist();
        assert!(persisted.is_some());
        assert!(self.get_id().is_some());
        let persisted_id = persisted.unwrap().get_id();
        assert!(persisted_id.is_some());
        let persisted_id = persisted_id.unwrap();
        let self_id = self.get_id().unwrap();
        assert_eq!(self_id, persisted_id);
        // Delete by id
        let deleted = self.delete();
        assert_eq!(deleted, Some(1));
        let just_deleted = Self::get(self_id);
        assert!(just_deleted.is_none());
        // Delete by self
        self.persist();
        let self_id = self.get_id().unwrap();
        let persisted = Self::get(self_id);
        assert!(persisted.is_some());
        let deleted = self.delete();
        assert_eq!(deleted, Some(1));
        let just_deleted = Self::get(self_id);
        assert!(just_deleted.is_none());
    }
}
