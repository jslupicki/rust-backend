extern crate actix_web;
extern crate cookie;
extern crate dao;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use actix_web::http::Cookie;
use actix_web::{server, App, Error, HttpRequest, HttpResponse};

mod employee;
mod session;
mod user;

fn index(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    info!("Got request!");
    Ok(HttpResponse::Ok()
        .cookie(Cookie::new("key", "value"))
        .content_type("text/html")
        .body("Hello world"))
}

fn main_app() -> App {
    App::new().resource("/", |r| r.f(index))
}

pub fn start() {
    info!("Start REST");

    let app_factory = || {
        vec![
            // Order of prefixes is important - should be from most specific to less.
            user::user_app("/users"),
            employee::employee_app("/employees"),
            session::session_app("/auth"),
            main_app(),
        ]
    };

    server::new(app_factory)
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
