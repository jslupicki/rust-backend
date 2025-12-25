use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::http::Method;
use dao::{Crud, EmployeeDTO, Searchable};

use crate::session::LoggedGuard::{Logged, LoggedAsAdmin};

async fn get_employees() -> Result<HttpResponse, Error> {
    let employees: Vec<EmployeeDTO> = EmployeeDTO::get_all();
    let body = serde_json::to_string(&employees)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn get_employee(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id: i32 = path.parse().unwrap();
    match EmployeeDTO::get(id) {
        Some(employee) => {
            let body = serde_json::to_string(&employee)?;
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body))
        }
        None => Err(ErrorNotFound(format!(
            "Can't find employee with id = {}",
            id
        ))),
    }
}

async fn update_employee(employee_json: Json<EmployeeDTO>) -> Result<HttpResponse, Error> {
    let mut employee = employee_json.clone();
    match employee.persist() {
        Some(employee) => {
            let body = serde_json::to_string(&employee)?;
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body))
        }
        None => Err(ErrorInternalServerError(format!(
            "Failed to update employee {:?}",
            employee_json
        ))),
    }
}

async fn delete_employee(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id: i32 = path.parse().unwrap();
    let employee = EmployeeDTO::get(id);
    match employee {
        Some(e) => match e.delete() {
            Some(n) if n == 1 => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(format!("Removed employee with id = {}", id))),
            Some(n) => Err(ErrorInternalServerError(format!(
                "Removed {} employees with id = {}",
                n, id
            ))),
            None => Err(ErrorInternalServerError("???")),
        },
        None => Err(ErrorNotFound(format!(
            "Not found employee with id = {}",
            id
        ))),
    }
}

async fn get_employee_template() -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn config(cfg: &mut web::ServiceConfig, prefix: &str) {
    cfg.service(
        web::resource(prefix)
            .wrap(LoggedAsAdmin(&[Method::PUT, Method::POST]))
            .route(web::get().to(get_employees))
            .route(web::put().to(update_employee))
            .route(web::post().to(update_employee)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/template"))
            .wrap(Logged)
            .route(web::get().to(get_employee_template)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/{id}"))
            .wrap(LoggedAsAdmin(&[Method::DELETE]))
            .route(web::get().to(get_employee))
            .route(web::delete().to(delete_employee)),
    );
}
