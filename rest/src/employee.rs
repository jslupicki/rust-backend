use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::http::Method;
use actix_web::web::Json;
use actix_web::{App, Error, HttpRequest, HttpResponse};
use actix_service::ServiceFactory;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use chrono::NaiveDate;

use dao::{Contact, Employee, NewContact, NewEmployee, NewSalary, Salary};

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

fn get_employees(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn get_employee(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn update_employee(employee_json: Json<EmployeeDTO>) -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn get_employee_template(_req: &HttpRequest) -> Result<HttpResponse, Error> {
    let body = "NOT YET IMPLEMENTED".to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

// TODO: replace by configure: https://docs.rs/actix-web/2.0.0/actix_web/struct.App.html#method.configure
pub fn employee_app(
    prefix: &str,
) -> App<
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
            r.method(Method::GET).f(get_employees);
            r.method(Method::PUT).with(update_employee);
            r.method(Method::POST).with(update_employee);
        })
        .resource("/template", |r| {
            r.method(Method::GET).f(get_employee_template)
        })
        .resource("{id}", |r| r.method(Method::GET).f(get_employee))
}
