use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use connection::get_connection;

/// Implement CRUD operations
pub trait Crud
where
    Self: Sized,
{
    type DbType: Into<Self>;

    /// Retrieve ID
    fn get_id(&self) -> Option<i32>;
    /// Update self from other - used in persist*()
    fn update(&mut self, other: &Self);
    /// Just retrieve T by id
    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<Self::DbType>;
    /// Save or update - as result should return just saved record (NOT self)  
    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<Self::DbType>;

    /// Save using provided connection - uses save_simple()
    fn save_in_transaction(&self, conn: &SqliteConnection) -> Option<Self> {
        conn.transaction(|| self.save_simple(conn))
            .optional()
            .unwrap_or(None)
            .map(|s: Self::DbType| s.into())
    }

    /// The same as save_in_transaction() but then update Self by result - useful when you want save new record without ID and update Self with ID from database
    fn persist_in_transaction(&mut self, conn: &SqliteConnection) -> Option<Self> {
        self.save_in_transaction(conn).map(|s| {
            self.update(&s);
            s
        })
    }

    /// Get by ID and provided connection
    fn get_with_conn(id_to_find: i32, conn: &SqliteConnection) -> Option<Self> {
        Self::get_simple(id_to_find, conn)
            .optional()
            .unwrap_or(None)
            .map(|s: Self::DbType| s.into())
    }

    /// Get by ID but it use default connection - uses get_with_conn()
    fn get(id_to_find: i32) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        Self::get_with_conn(id_to_find, conn)
    }

    /// Save but it use default connection - uses save_in_transaction()
    fn save(&self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.save_in_transaction(conn)
    }

    /// Persist but it use default connection - uses persist_in_transaction()
    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.persist_in_transaction(conn)
    }
}
