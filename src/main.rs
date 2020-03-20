extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}
