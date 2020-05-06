use std::env;
use std::sync::Mutex;

use diesel::sqlite::SqliteConnection;
use diesel_migrations;

use dao::{get_connection, initialize_db};

lazy_static! {
    pub static ref MUTEX: Mutex<i32> = Mutex::new(0i32);
}

#[macro_export]
macro_rules! setup_test {
    ($test_name:expr) => {
        #[allow(unused_variables)]
        let lock = commons_for_tests::MUTEX.lock();
        commons_for_tests::initialize_log();
        info!("Start {}() test", $test_name);
        commons_for_tests::setup_db();
        defer! {
            commons_for_tests::tear_down_db();
            info!("End {}() test", $test_name);
        }
    };
}

pub fn initialize_log() {
    let _ = log4rs::init_file("log4rs.yml", Default::default());
}

pub fn setup_db() {
    info!("Initialize DB (if not exist), run migrations");
    env::set_var("DATABASE_URL", ":memory:");
    env::set_var("POOL_SIZE", "1");
    initialize_db().unwrap();
}

pub fn tear_down_db() {
    let conn: &SqliteConnection = &get_connection();
    loop {
        match diesel_migrations::revert_latest_migration(conn) {
            Ok(migration) => info!("Reverted {}", migration),
            Err(e) => {
                info!("Reverted all migrations: {}", e);
                break;
            }
        };
    }
}
