use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::http::Method;
use actix_web::{App, Error, HttpRequest, HttpResponse};
use actix_web::web::{Json};
use actix_service::ServiceFactory;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};

use dao::{NewUser, User};

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

fn get_user(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let id: i32 = req.match_info().query("id").parse().unwrap();
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

fn update_user(user_json: Json<UserDTO>) -> Result<HttpResponse, Error> {
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

fn get_user_template(_req: &HttpRequest) -> Result<HttpResponse, Error> {
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

// TODO: replace by configure: https://docs.rs/actix-web/2.0.0/actix_web/struct.App.html#method.configure
pub fn user_app(prefix: &str) -> App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
    >,
    impl MessageBody,
> {
    App::new()
        .prefix(prefix)
        .resource("", |r| {
            r.method(Method::GET).f(get_users);
            r.method(Method::PUT).with(update_user);
            r.method(Method::POST).with(update_user);
        })
        .resource("/template", |r| r.method(Method::GET).f(get_user_template))
        .resource("{id}", |r| r.method(Method::GET).f(get_user))
}
