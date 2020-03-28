extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}

#[cfg(test)]
mod tests {
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations;
    use super::*;
    use std::sync::Mutex;
    use std::{thread, time};
    use dao::{get_connection, initialize_db};
    use std::env;

    lazy_static! {
        static ref MUTEX: Mutex<i32> = Mutex::new(0i32);
    }

    #[test]
    fn integration_test1() {
        initialize_log();
        perform_test("Integration Test1");
    }

    #[test]
    fn integration_test2() {
        initialize_log();
        perform_test("Integration Test2");
    }

    #[test]
    fn integration_test3() {
        initialize_log();
        perform_test("Integration Test3");
    }

    fn initialize_log() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
    }

    #[allow(unused_variables)]
    fn perform_test(name: &str) {
        info!("Start {}", name);
        let lock = MUTEX.lock();
        setup_db();
        let timeout = time::Duration::from_millis(200);
        for i in 1..10 {
            debug!("In {}: {}", name, i);
            thread::sleep(timeout);
        }
        tear_down_db();
        info!("End of {}", name);
    }

    fn setup_db() {
        info!("Initialize DB (if not exist), run migrations");
        env::set_var("DATABASE_URL", ":memory:");
        env::set_var("POOL_SIZE", "1");
        initialize_db().unwrap();    
    }

    fn tear_down_db() {
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
}
