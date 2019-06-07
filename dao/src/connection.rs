use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::io::stdout;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<SqliteConnection>> = create_connection_pool();
}

embed_migrations!("./migrations");

fn create_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn initialize_db() -> Result<(), RunMigrationsError> {
    let conn: &SqliteConnection = &POOL.get().unwrap();
    info!("Initialize DB (if not exist), run migrations");
    embedded_migrations::run_with_output(conn, &mut stdout())
}
