extern crate actix_web;
extern crate cookie;
extern crate dao;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{App, Error, HttpRequest, HttpResponse, server};
use cookie::Cookie;

mod session;
mod user;


fn index(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    info!("Got request!");
    Ok(
        HttpResponse::Ok()
            .cookie(Cookie::new("key", "value"))
            .content_type("text/html")
            .body("Hello world")
    )
}

pub fn start() {
    info!("Start application");

    let apps = || vec![
        App::new().resource("/", |r| r.f(index)),
        user::user_app()
    ];

    server::new(apps)
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
