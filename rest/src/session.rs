use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::error::ErrorUnauthorized;
use actix_web::http::{Cookie, Method};
use actix_web::middleware::{Middleware, Started};
use actix_web::{App, Error, HttpRequest, HttpResponse, Json};
use uuid::Uuid;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub struct Headers;

impl<S> Middleware<S> for Headers {
    /// Method is called when request is ready. It may return
    /// future, which should resolve before next middleware get called.
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, Error> {
        let session = &req
            .cookie("session")
            .map_or("nothing".to_string(), |c| c.value().to_string());
        if let Some(username) = SESSIONS.lock().unwrap().get(session) {
            info!(
                "Allow access to {} with session {} for user {}",
                req.path(),
                session,
                username
            );
            Ok(Started::Done)
        } else {
            error!(
                "Unauthorized access to {} with session {}",
                req.path(),
                session
            );
            Err(ErrorUnauthorized(format!(
                "You are not authorized to access '{}'",
                req.path()
            )))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LoginDTO {
    username: String,
    password: String,
}

fn login(body: Json<LoginDTO>) -> Result<HttpResponse, Error> {
    info!(
        "Try to login '{}' with password '{}'",
        &body.username, &body.password
    );
    for user in dao::get_users() {
        debug!(
            "There is user '{}' with password '{}' - admin {}",
            user.username, user.password, user.is_admin
        );
    }
    if dao::validate_user(&body.username, &body.password) {
        let session_value = Uuid::new_v4().to_hyphenated().to_string();
        let session_cookie = Cookie::new("session", session_value.to_owned());
        let mut response = HttpResponse::Ok().content_type("text/plain").body(format!(
            "Login '{}' with password '{}' - session '{}'",
            &body.username, &body.password, &session_value
        ));
        response.add_cookie(&session_cookie)?;
        SESSIONS
            .lock()
            .unwrap()
            .insert(session_value, body.username.to_owned());
        Ok(response)
    } else {
        Err(ErrorUnauthorized("Wrong login or password"))
    }
}

fn logout(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let session = req
        .cookie("session")
        .unwrap_or_else(|| Cookie::new("n", "not exist"));
    info!("Logout from session '{}'", session.value());
    SESSIONS.lock().unwrap().remove(session.value());
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Logout from session '{}'", session.value())))
}

fn get_login_template(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let login = LoginDTO {
        username: "".to_string(),
        password: "".to_string(),
    };
    let body = serde_json::to_string(&login)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn session_app(prefix: &str) -> App {
    App::new()
        .prefix(prefix)
        .resource("", |r| {
            r.method(Method::POST).with(login);
            r.method(Method::DELETE).f(logout);
        })
        .resource("/template", |r| r.method(Method::GET).f(get_login_template))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_test() {
        let mut sessions = SESSIONS.lock().unwrap();
        sessions.insert("key".to_string(), "value".to_string());
        assert_eq!(
            &"value".to_string(),
            sessions.get(&"key".to_string()).unwrap()
        );
    }
}
