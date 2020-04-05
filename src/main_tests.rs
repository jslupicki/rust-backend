use std::env;
use std::sync::Mutex;

use actix_http::http::{Cookie, StatusCode};
use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App};
use bytes::Bytes;
use diesel::sqlite::SqliteConnection;
use diesel_migrations;
use dao::{get_connection, initialize_db};
use rest::LoginDTO;

use crate::test_data::URLS;

lazy_static! {
    pub static ref MUTEX: Mutex<i32> = Mutex::new(0i32);
}

#[actix_rt::test]
async fn call_to_index_should_return_hello_world() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start call_to_index_should_return_hello_world() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End call_to_index_should_return_hello_world() test");
    }

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    let result = test::read_body(resp).await;
    assert_eq!(result, Bytes::from_static(b"Hello world"));
}

#[actix_rt::test]
async fn login_with_correct_credentials() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start login_with_correct_credentials() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End login_with_correct_credentials() test");
    }

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let credentials = LoginDTO {
        username: String::from("admin"),
        password: String::from("fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b"),
    };
    let req = test::TestRequest::post()
        .uri("/auth")
        .set_json(&credentials)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let session = resp.response().cookies().find(|c| c.name() == "session");

    assert!(resp.status().is_success());
    assert!(session.is_some());
}

#[actix_rt::test]
async fn login_with_incorrect_credentials() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start login_with_incorrect_credentials() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End login_with_incorrect_credentials() test");
    }

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let credentials = LoginDTO {
        username: String::from("admin"),
        password: String::from("wrong password"),
    };
    let req = test::TestRequest::post()
        .uri("/auth")
        .set_json(&credentials)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    let session = resp.response().cookies().find(|c| c.name() == "session");

    assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
    assert!(session.is_none());
}

#[actix_rt::test]
async fn check_access_control() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start check_access_control() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End check_access_control() test");
    }

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login(
        "admin",
        "fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b",
        &mut app,
    )
    .await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        for url in &*URLS {
            debug!("Checking {:#?}", url);
            let req_without_session = test::TestRequest::get()
                .uri(url.url)
                .method(url.method.clone())
                .to_request();
            let req_with_session = test::TestRequest::get()
                .uri(url.url)
                .method(url.method.clone())
                .cookie(session.clone())
                .to_request();
            let resp_without_session = test::call_service(&mut app, req_without_session).await;
            let resp_with_session = test::call_service(&mut app, req_with_session).await;
            if url.guarded {
                assert_eq!(
                    StatusCode::UNAUTHORIZED,
                    resp_without_session.status(),
                    "Call {:#?} without session should respond with UNAUTHORIZED",
                    url
                );
                assert_ne!(
                    StatusCode::UNAUTHORIZED,
                    resp_with_session.status(),
                    "Call {:#?} with session should NOT respond with UNAUTHORIZED",
                    url
                );
            } else {
                assert_ne!(
                    StatusCode::UNAUTHORIZED,
                    resp_without_session.status(),
                    "Call {:#?} without session should NOT respond with UNAUTHORIZED",
                    url
                );
                assert_ne!(
                    StatusCode::UNAUTHORIZED,
                    resp_with_session.status(),
                    "Call {:#?} with session should NOT respond with UNAUTHORIZED",
                    url
                );
            }
        }
    }
}

pub fn initialize_log() {
    let _ = log4rs::init_file("log4rs.yml", Default::default());
}

pub fn setup_db() {
    info!("Initialize DB (if not exist), run migrations");
    env::set_var("DATABASE_URL", ":memory:");
    env::set_var("POOL_SIZE", "1");
    initialize_db().unwrap();
}

pub fn tear_down_db() {
    let conn: &SqliteConnection = &get_connection();
    loop {
        match diesel_migrations::revert_latest_migration(conn) {
            Ok(migration) => info!("Reverted {}", migration),
            Err(e) => {
                info!("Reverted all migrations: {}", e);
                break;
            }
        };
    }
}

pub async fn login<S, B, E>(username: &str, password: &str, app: &mut S) -> Option<Cookie<'static>>
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let credentials = LoginDTO {
        username: String::from(username),
        password: String::from(password),
    };
    let req = test::TestRequest::post()
        .uri("/auth")
        .set_json(&credentials)
        .to_request();
    let resp = test::call_service(app, req).await;
    resp.response()
        .cookies()
        .find(|c| c.name() == "session")
        .map(|c| c.into_owned())
}

pub async fn login_as_admin<S, B, E>(app: &mut S) -> Option<Cookie<'static>>
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    login(
        "admin", 
        "fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b", 
        app
    ).await
}

pub async fn login_as_user<S, B, E>(app: &mut S) -> Option<Cookie<'static>>
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    login(
        "user", 
        "8ac76453d769d4fd14b3f41ad4933f9bd64321972cd002de9b847e117435b08b", 
        app
    ).await
}
