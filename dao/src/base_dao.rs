use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use connection::get_connection;

pub trait Crud<T>
where
    Self: Sized + From<T>,
{
    fn update(&mut self, other: &Self);
    fn get_simple(id_to_find: i32, conn: &SqliteConnection) -> QueryResult<T>;
    fn save_simple(&self, conn: &SqliteConnection) -> QueryResult<T>;

    fn save_in_transaction(&self, conn: &SqliteConnection) -> Option<Self> {
        conn.transaction(|| self.save_simple(conn))
            .optional()
            .unwrap_or(None)
            .map(|s: T| s.into())
    }

    fn persist_in_transaction(&mut self, conn: &SqliteConnection) -> Option<Self> {
        self.save_in_transaction(conn).map(|s| {
            self.update(&s);
            s
        })
    }

    fn get_with_conn(id_to_find: i32, conn: &SqliteConnection) -> Option<Self> {
        Self::get_simple(id_to_find, conn)
            .optional()
            .unwrap_or(None)
            .map(|s: T| s.into())
    }

    fn get(id_to_find: i32) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        Self::get_with_conn(id_to_find, conn)
    }

    fn save(&self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.save_in_transaction(conn)
    }

    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &get_connection();
        self.persist_in_transaction(conn)
    }
}
