extern crate actix_web;
extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;

use actix_web::{App, HttpRequest, server};

use dao::test;

fn index(_req: &HttpRequest) -> &'static str {
    info!("Got request!");
    "Hello world!"
}

pub fn start() {
    info!("Start application");

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
