extern crate actix_web;
extern crate actix_service;
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
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_service::ServiceFactory;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};

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

// TODO: replace by configure: https://docs.rs/actix-web/2.0.0/actix_web/struct.App.html#method.configure
fn main_app() -> App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
    >,
    impl MessageBody,
> {
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

    HttpServer::new(app_factory)
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
