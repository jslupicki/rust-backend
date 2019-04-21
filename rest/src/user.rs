use std::ops::Deref;

use actix_web::{App, Error, HttpRequest, HttpResponse};
use actix_web::http::Method;
use actix_web::middleware::{Middleware, Started};

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

pub fn user_app(prefix: &str) -> App {
    App::new()
        .middleware(Headers)
        .prefix(prefix)
        .resource("", |r| r.method(Method::GET).f(get_users))
        .resource("/template", |r| r.method(Method::GET).f(get_user_template))
}
