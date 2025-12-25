use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

lazy_static! {
    static ref POOL: Pool<ConnectionManager<SqliteConnection>> = create_connection_pool();
}

fn create_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => String::from("rust_backend.sqlite3"),
    };
    let pool_size = match env::var("POOL_SIZE") {
        Ok(pool_size) => pool_size.parse::<u32>().unwrap(),
        Err(_) => 1,
    };
    info!(
        "Initialize connection pool with DATABASE_URL='{}', POOL_SIZE='{}'",
        database_url, pool_size
    );
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn initialize_db() {
    let mut conn = get_connection();
    info!("Initialize DB (if not exist), run migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Fail to initiate DB");
}

pub fn get_connection() -> r2d2::PooledConnection<ConnectionManager<SqliteConnection>> {
    POOL.get().unwrap()
}
