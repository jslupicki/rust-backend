extern crate dao;

#[macro_use]
extern crate log;
extern crate actix_web;
extern crate log4rs;

use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    info!("Got request!");
    "Hello world!"
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
