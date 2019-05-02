extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start();
}
