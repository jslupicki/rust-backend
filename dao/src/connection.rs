use std::env;
use std::io::stdout;

use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<SqliteConnection>> = create_connection_pool();
}

embed_migrations!("./migrations");

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
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn initialize_db() -> Result<(), RunMigrationsError> {
    let conn: &SqliteConnection = &get_connection();
    info!("Initialize DB (if not exist), run migrations");
    embedded_migrations::run_with_output(conn, &mut stdout())
}

pub fn get_connection() -> r2d2::PooledConnection<ConnectionManager<SqliteConnection>> {
    POOL.get().unwrap()
}
