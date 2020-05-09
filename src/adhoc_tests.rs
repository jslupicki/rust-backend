use actix_http::http::Method;
use actix_web::{test, App};

use crate::commons_for_tests;
use crate::main_tests::login_as_admin;

#[actix_rt::test]
async fn check_login_guard() {
    setup_test!("check_login_guard");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_admin(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        let req = test::TestRequest::get()
            .uri("/users/template")
            .method(Method::GET)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        info!("Hello");
        info!("GET /users/template => {:#?}", resp.status());
    }
}

#[test]
fn check_lg() {
    enum LG {
        Logged,
        LoggedAsAdmin(&'static [Method]),
    }

    use LG::{Logged, LoggedAsAdmin};

    fn show_lg(guard: &LG) {
        match guard {
            Logged => info!("Just logged"),
            LoggedAsAdmin(methods) => info!("Logged as admin for methods: {:?}", methods),
        };
    }

    let _ = log4rs::init_file("log4rs.yml", Default::default());
    let logged = Logged;
    let logged_as_admin = LoggedAsAdmin(&[Method::POST, Method::PUT]);

    let guard = &logged;
    info!("First just Logged:");
    show_lg(guard);

    let guard = &logged_as_admin;
    info!("Second LoggedAsAdmin:");
    show_lg(guard);
}
