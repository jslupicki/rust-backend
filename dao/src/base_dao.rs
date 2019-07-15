use connection::POOL;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

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

    fn save(&self) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        self.save_in_transaction(conn)
    }

    fn persist(&mut self) -> Option<Self> {
        let conn: &SqliteConnection = &POOL.get().unwrap();
        self.persist_in_transaction(conn)
    }
}
