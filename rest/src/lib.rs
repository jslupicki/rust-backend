extern crate actix_service;
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
use actix_web::{web, App, Error, HttpResponse, HttpServer};

#[macro_use]
mod session;
mod employee;
mod user;

pub use session::LoginDTO;
pub use user::UserDTO;

async fn index() -> Result<HttpResponse, Error> {
    info!("Got request!");
    Ok(HttpResponse::Ok()
        .cookie(Cookie::new("key", "value"))
        .content_type("text/html")
        .body("Hello world"))
}

pub fn config(cfg: &mut web::ServiceConfig, prefix: &str) {
    cfg.service(web::resource(prefix).route(web::get().to(index)));
}

pub fn config_all(cfg: &mut web::ServiceConfig) {
    user::config(cfg, "/users");
    employee::config(cfg, "/employees");
    session::config(cfg, "/auth");
    config(cfg, "/");
}

pub async fn start() -> std::io::Result<()> {
    info!("Start REST");

    HttpServer::new(|| App::new().configure(|cfg| config_all(cfg)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
