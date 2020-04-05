extern crate dao;
#[cfg(test)]
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;
#[cfg(test)]
#[macro_use]
extern crate scopeguard;

#[cfg(test)]
mod test_data;
#[cfg(test)]
mod main_tests;
#[cfg(test)]
mod employee_tests;
#[cfg(test)]
mod user_tests;
#[cfg(test)]
mod adhoc_tests;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}
