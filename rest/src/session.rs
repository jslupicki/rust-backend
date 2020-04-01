use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::Cookie;
use actix_web::web::Json;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web::guard::Guard;
use actix_http::RequestHead;

use uuid::Uuid;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
pub struct LoginGuard;

impl Guard for LoginGuard {
    fn check(&self, req: &RequestHead) -> bool {
        match req.headers().get("cookie") {
            Some(cookies) => {
                info!("LoginGuard: I find cookies in header: {:#?}", cookies);
                match cookies.to_str() {
                    Ok(cookies_as_str) => {
                        let cookies_vec: Vec<&str> = cookies_as_str.split(";").collect();
                        let session = cookies_vec.iter()
                            .map(|c| Cookie::parse(c.trim()))
                            .find(|c| match c {
                                Ok(c) => c.name().eq("session"),
                                Err(_) => false,
                            })
                            .map_or("nothing".to_string(), |c| c.unwrap().value().to_string())
                            ;
                        if let Some(username) = SESSIONS.lock().unwrap().get(&session) {
                            info!(
                                "LoginGuard: Allow access to {} with session {} for user {}",
                                req.uri,
                                session,
                                username
                            );
                            return true;                    
                        } else {
                            error!(
                                "LoginGuard: Unauthorized access to {} with session {}",
                                req.uri,
                                session
                            );
                            return false;                                            
                        }
                    },
                    Err(_) => return false,
                }
            },
            None => {
                info!("LoginGuard: I can't find cookies in header - reject access to {}", req.uri);
                return false;
            },
        };
    }
}

pub fn is_logged(req: &ServiceRequest) -> bool {
    let session = req
        .cookie("session")
        .map_or("nothing".to_string(), |c| c.value().to_string());
    if let Some(username) = SESSIONS.lock().unwrap().get(&session) {
        info!(
            "Allow access to {} with session {} for user {}",
            req.path(),
            session,
            username
        );
        true
    } else {
        error!(
            "Unauthorized access to {} with session {}",
            req.path(),
            session
        );
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

async fn login(body: Json<LoginDTO>) -> Result<HttpResponse, Error> {
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

async fn logout(req: HttpRequest) -> Result<HttpResponse, Error> {
    let session = req
        .cookie("session")
        .unwrap_or_else(|| Cookie::new("n", "not exist"));
    info!("Logout from session '{}'", session.value());
    SESSIONS.lock().unwrap().remove(session.value());
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Logout from session '{}'", session.value())))
}

async fn get_login_template() -> Result<HttpResponse, Error> {
    let login = LoginDTO {
        username: "".to_string(),
        password: "".to_string(),
    };
    let body = serde_json::to_string(&login)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn config(cfg: &mut web::ServiceConfig, prefix: &str) {
    cfg.service(
        web::resource(prefix)
            .route(web::post().to(login))
            .route(web::delete().to(logout)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/template"))
            .route(web::get().to(get_login_template)),
    );
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
