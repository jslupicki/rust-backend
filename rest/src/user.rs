use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::middleware::Logger;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse};
use dao::{NewUser, User};

use crate::session;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserDTO {
    id: Option<i32>,
    username: Option<String>,
    password: Option<String>,
    is_admin: Option<bool>,
}

impl From<User> for UserDTO {
    fn from(u: User) -> Self {
        UserDTO {
            id: Some(u.id),
            username: Some(u.username),
            password: Some(u.password),
            is_admin: Some(u.is_admin),
        }
    }
}

impl From<UserDTO> for NewUser {
    fn from(u: UserDTO) -> Self {
        NewUser {
            username: u.username.unwrap_or("".to_string()),
            password: u.password.unwrap_or("".to_string()),
            is_admin: u.is_admin.unwrap_or(false),
        }
    }
}

impl UserDTO {
    fn update_user(&self, user: &mut User) {
        if let Some(username) = &self.username {
            user.username = username.clone()
        };
        if let Some(password) = &self.password {
            user.password = password.clone()
        };
        if let Some(is_admin) = &self.is_admin {
            user.is_admin = is_admin.clone()
        };
    }
}

async fn get_users() -> Result<HttpResponse, Error> {
    let users: Vec<UserDTO> = dao::get_users()
        .into_iter()
        .map(|u| UserDTO::from(u))
        .collect();
    let body = serde_json::to_string(&users)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn get_user(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id: i32 = path.parse().unwrap();
    match dao::get_user(id) {
        Some(user) => {
            let body = serde_json::to_string(&UserDTO::from(user))?;
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body))
        }
        None => Err(ErrorNotFound(format!("Can't find user with id = {}", id))),
    }
}

async fn update_user(user_json: Json<UserDTO>) -> Result<HttpResponse, Error> {
    let user = user_json.clone();
    let result = if let Some(id) = user.id {
        let mut existing_user = dao::get_user(id).unwrap();
        user.update_user(&mut existing_user);
        dao::update_user(&existing_user)
    } else {
        dao::create_user(&NewUser::from(user))
    };
    if result.is_ok() {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&result.unwrap())?))
    } else {
        Err(ErrorInternalServerError(format!(
            "Failed to update user {:?} because {:?}",
            user_json,
            result.unwrap_err()
        )))
    }
}

async fn get_user_template() -> Result<HttpResponse, Error> {
    let user = UserDTO {
        id: Some(1i32),
        username: Some("".to_string()),
        password: Some("".to_string()),
        is_admin: Some(false),
    };
    let body = serde_json::to_string(&user)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn config(cfg: &mut web::ServiceConfig, prefix: &str) {
    cfg.service(
        web::resource(prefix)
            .wrap_fn(|req, srv| {
                if session::is_logged(&req) {
                    srv.call(req)
                } else {
                    // TODO: implement correct response when user is not logged.
                    srv.call(req)
                }
            })
            .route(web::get().to(get_users))
            .route(web::put().to(update_user))
            .route(web::post().to(update_user)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/template")).route(web::get().to(get_user_template)),
    );
    cfg.service(web::resource(format!("{}{}", prefix, "/{id}}")).route(web::get().to(get_user)));
}
