extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;
#[macro_use]
extern crate lazy_static;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}

#[cfg(test)]
mod tests {
   
    use super::*;
    use std::sync::Mutex;
    use std::{thread, time};

    lazy_static! {
        static ref MUTEX: Mutex<i32> = Mutex::new(0i32);
    }

    #[test]
    fn test1() {
        initialize_log();
        perform_test("Test1");
    }

    #[test]
    fn test2() {
        initialize_log();
        perform_test("Test2");
    }

    #[test]
    fn test3() {
        initialize_log();
        perform_test("Test3");
    }

    fn initialize_log() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
    }

    #[allow(unused_variables)]
    fn perform_test(name: &str) {
        info!("Start {}", name);
        let lock = MUTEX.lock();
        let timeout = time::Duration::from_millis(200);
        for i in 1..10 {
            debug!("In {}: {}", name, i);
            thread::sleep(timeout);
        }
        info!("End of {}", name);
    }
}
