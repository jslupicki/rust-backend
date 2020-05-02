use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};

use actix_http::Response;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::Cookie;
use actix_web::web::Json;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::future::{ok, Ready};
use uuid::Uuid;

lazy_static! {
    //TODO: should hold ID also
    static ref SESSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub struct LoggedGuard {
    pub have_to_be_admin: bool,
}

impl<S> Transform<S> for LoggedGuard
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = LoggedGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggedGuardMiddleware {
            service,
            have_to_be_admin: self.have_to_be_admin,
        })
    }
}

pub struct LoggedGuardMiddleware<S> {
    service: S,
    have_to_be_admin: bool,
}

impl<S> Service for LoggedGuardMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if is_logged(&req, self.have_to_be_admin) {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(ServiceResponse::new(
                    req.into_parts().0,
                    Response::Unauthorized().finish(),
                ))
            })
        }
    }
}

pub fn is_logged(req: &ServiceRequest, have_to_be_admin: bool) -> bool {
    let session = req
        .cookie("session")
        .map_or("nothing".to_string(), |c| c.value().to_string());
    if let Some(username) = SESSIONS.lock().unwrap().get(&session) {
        if have_to_be_admin {
            //TODO: add check if 'username' is admin
        }
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
    //TODO: let Some(user) = dao::validate_user(&body.username, &body.password) when dao will be fixed
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
            .insert(session_value, body.username.to_owned()); //TODO: put ID also
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
