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

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::{App, Error, HttpRequest, HttpResponse, server};
use actix_web::http::Method;
use actix_web::middleware::{Middleware, Started};
use cookie::Cookie;

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
        let a = req.cookie("a");
        info!("Cookie a={:?}", a);
        for c in req.cookies().unwrap().deref() {
            info!("Cookie: {:?}", c);
        }
        Ok(Started::Done)
    }
}

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn index(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    info!("Got request!");
    Ok(
        HttpResponse::Ok()
            .cookie(Cookie::new("key", "value"))
            .content_type("text/html")
            .body("Hello world")
    )
}

fn get_users(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let users: Vec<UserDTO> = dao::get_users()
        .into_iter()
        .map(|u| UserDTO { username: u.username })
        .collect()
        ;
    let body = serde_json::to_string(&users)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn get_user_template(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let user = UserDTO { username: "".to_string() };
    let body = serde_json::to_string(&user)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn start() {
    info!("Start application");

    let apps = || vec![
        App::new()
            .middleware(Headers)
            .resource("/", |r| r.f(index))
            .resource("/users", |r| r.method(Method::GET).f(get_users))
            .resource("/users/template", |r| r.method(Method::GET).f(get_user_template))
    ];

    server::new( apps)
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_test() {
        let mut sessions = SESSIONS.lock().unwrap();
        sessions.insert("key".to_string(), "value".to_string());
        assert_eq!(&"value".to_string(), sessions.get(&"key".to_string()).unwrap());
    }
}
