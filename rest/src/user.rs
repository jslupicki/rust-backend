use actix_web::http::Method;
use actix_web::{App, Error, HttpRequest, HttpResponse};

use dao::{NewUser, User};

use crate::session::Headers;

#[derive(Serialize, Deserialize)]
struct UserDTO {
    username: String,
    password: String,
    is_admin: bool,
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        UserDTO {
            username: u.username,
            password: u.password,
            is_admin: u.is_admin,
        }
    }
}

fn get_users(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let users: Vec<UserDTO> = dao::get_users()
        .into_iter()
        .map(|u| UserDTO::from(u))
        .collect();
    let body = serde_json::to_string(&users)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn get_user_template(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let user = UserDTO {
        username: "".to_string(),
        password: "".to_string(),
        is_admin: false,
    };
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
