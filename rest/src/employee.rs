use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse};
use chrono::NaiveDate;

use crate::session::LoggedGuard;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SalaryDTO {
    id: Option<i32>,
    employee_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    amount: i64,
    search_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ContactDTO {
    id: Option<i32>,
    employee_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    phone: String,
    address: Option<String>,
    search_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EmployeeDTO {
    id: Option<i32>,
    first_name: Option<String>,
    last_name: Option<String>,
    search_string: Option<String>,
    salaries: Vec<SalaryDTO>,
    contacts: Vec<ContactDTO>,
}

async fn get_employees() -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn get_employee() -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn update_employee(_employee_json: Json<EmployeeDTO>) -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn delete_employee(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id: i32 = path.parse().unwrap();
    info!("Not yet implemented delete user: {}", id);
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
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
            .wrap(LoggedGuard {
                have_to_be_admin: false,
            })
            .route(web::get().to(get_employees))
            .wrap(LoggedGuard {
                have_to_be_admin: true,
            })
            .route(web::put().to(update_employee))
            .route(web::post().to(update_employee)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/template"))
            .wrap(LoggedGuard {
                have_to_be_admin: false,
            })
            .route(web::get().to(get_employee_template)),
    );
    cfg.service(
        web::resource(format!("{}{}", prefix, "/{id}"))
            .wrap(LoggedGuard {
                have_to_be_admin: false,
            })
            .route(web::get().to(get_employee))
            .wrap(LoggedGuard {
                have_to_be_admin: true,
            })
            .route(web::delete().to(delete_employee)),
    );
}
