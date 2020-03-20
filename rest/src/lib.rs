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
use actix_web::{App, Error, HttpResponse, HttpServer, web};

mod employee;
mod session;
mod user;

async fn index() -> Result<HttpResponse, Error> {
    info!("Got request!");
    Ok(HttpResponse::Ok()
        .cookie(Cookie::new("key", "value"))
        .content_type("text/html")
        .body("Hello world"))
}

pub fn config(cfg: &mut web::ServiceConfig, prefix: &str) {
    cfg.service(web::resource(prefix)
        .route(web::get().to(index))
    );
}

pub async fn start() -> std::io::Result<()> {
    info!("Start REST");

    HttpServer::new(||
            App::new()
                .configure(|cfg| user::config(cfg, "/users"))
                .configure(|cfg| employee::config(cfg, "/employees"))
                .configure(|cfg| session::config(cfg, "/auth"))
                .configure(|cfg| config(cfg, "/"))
        )
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
