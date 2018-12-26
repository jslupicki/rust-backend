extern crate actix_web;
extern crate dao;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{App, Error, HttpRequest, HttpResponse, Responder, server};
use actix_web::http::Method;

use dao::test;

#[derive(Serialize, Deserialize)]
struct UserDTO {
    username: String
}

fn index(_req: &HttpRequest) -> &'static str {
    info!("Got request!");
    "Hello world!"
}

fn get_users(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let users = vec![
        UserDTO { username: "Test1".to_string() },
        UserDTO { username: "Test2".to_string() },
        UserDTO { username: "Test3".to_string() },
    ];
    let body = serde_json::to_string(&users)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn start() {
    info!("Start application");

    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource("/users", |r| r.method(Method::GET).f(get_users))
    )
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
