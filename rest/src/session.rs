use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};

use actix_http::http::Method;
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
    /// Map: session_id -> (username, user_id)
    static ref SESSIONS: Mutex<HashMap<String, (String, i32)>> = Mutex::new(HashMap::new());
}

pub enum LoggedGuard {
    Logged,
    LoggedWithException(&'static [Method]),
    LoggedAsAdmin(&'static [Method]),
    LoggedAsAdminWithException(&'static [Method], &'static [Method]),
}

use LoggedGuard::{Logged, LoggedWithException, LoggedAsAdmin, LoggedAsAdminWithException};

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
        match *self {
            Logged => {
                ok(LoggedGuardMiddleware {
                    service,
                    as_admin: &[],
                    except: &[],
                })
            },
            LoggedWithException(except) => {
                ok(LoggedGuardMiddleware {
                    service,
                    as_admin: &[],
                    except: except,
                })
            },
            LoggedAsAdmin(as_admin) => {
                ok(LoggedGuardMiddleware {
                    service,
                    as_admin: as_admin,
                    except: &[],
                })
            },
            LoggedAsAdminWithException(as_admin, except) => {
                ok(LoggedGuardMiddleware {
                    service,
                    as_admin: as_admin,
                    except: except,
                })
            }
        }
    }
}

pub struct LoggedGuardMiddleware<S> {
    service: S,
    as_admin: &'static [Method],
    except: &'static [Method],
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
        if is_logged(&req, self.as_admin, self.except) {
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

pub fn is_logged(req: &ServiceRequest, as_admin: &[Method], except: &[Method]) -> bool {
    if contain_method(req.method(), except) {
        return true;
    }
    let session = req
        .cookie("session")
        .map_or("nothing".to_string(), |c| c.value().to_string());
    if let Some((username, id)) = SESSIONS.lock().unwrap().get(&session) {
        let user = dao::get_user(*id).unwrap();
        let method = req.method();
        debug!(
            "session: {}, user: {}, is_admin: {}, method: {}, admin rights for methods: {:?}",
            session, user.username, user.is_admin, method, as_admin
        );
        let is_admin_method = contain_method(method, as_admin);
        if as_admin.len() == 0 || !is_admin_method || (is_admin_method && user.is_admin) {
            info!(
                "Allow access to {} with session {} for user '{}'",
                req.path(),
                session,
                username
            );
            true
        } else {
            error!(
                "Unauthorized access to {} with session {} for user '{}'",
                req.path(),
                session,
                username
            );
            false
        }
    } else {
        error!(
            "Unauthorized access to {} with session {}",
            req.path(),
            session
        );
        false
    }
}

fn contain_method(method: &Method, methods: &[Method]) -> bool {
    for m in methods.iter() {
        if method == m {
            return true;
        }
    }
    false
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
    if let Some(user) = dao::validate_user(&body.username, &body.password) {
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
            .insert(session_value, (body.username.to_owned(), user.id));
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
            .wrap(LoggedWithException(&[Method::POST]))
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
        sessions.insert("key".to_string(), ("value".to_string(), 1));
        assert_eq!(
            &"value".to_string(),
            &sessions.get(&"key".to_string()).unwrap().0
        );
    }
}
