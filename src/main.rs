extern crate dao;
#[cfg(test)]
extern crate diesel;
#[cfg(test)]
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
#[macro_use]
mod commons_for_tests;
#[cfg(test)]
mod adhoc_tests;
#[cfg(test)]
mod employee_tests;
#[cfg(test)]
mod main_tests;
#[cfg(test)]
mod user_tests;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db();
    rest::start().await
}
