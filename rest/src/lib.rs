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
use actix_web::middleware::{Middleware, Response, Started};

#[derive(Serialize, Deserialize)]
struct UserDTO {
    username: String
}

struct Headers;

impl<S> Middleware<S> for Headers {

    /// Method is called when request is ready. It may return
    /// future, which should resolve before next middleware get called.
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, Error> {
        info!("GOT REQUEST for {}", req.path());
        Ok(Started::Done)
    }
}


fn index(_req: &HttpRequest) -> &'static str {
    info!("Got request!");
    "Hello world!"
}

fn get_users(_req: &HttpRequest) -> Result<HttpResponse, Error> {
/*
    let users = vec![
        UserDTO { username: "Test1".to_string() },
        UserDTO { username: "Test2".to_string() },
        UserDTO { username: "Test3".to_string() },
    ];
*/
    let users: Vec<UserDTO> = dao::get_users()
        .into_iter()
        .map(|u| UserDTO { username: u.username})
        .collect()
        ;
    let body = serde_json::to_string(&users)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn start() {
    info!("Start application");

    server::new(|| App::new()
        .middleware(Headers)
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
