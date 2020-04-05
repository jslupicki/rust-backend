use crate::main_tests::{MUTEX, initialize_log, setup_db, tear_down_db, login_as_admin};
use actix_http::http::{Method};
use actix_web::{test, App};

#[actix_rt::test]
async fn check_login_guard() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start check_login_guard() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End check_login_guard() test");
    }

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_admin(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        let req = test::TestRequest::get()
            .uri("/users/template")
            .method(Method::GET)
            //.cookie(session.clone())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        info!("Hello");
        info!("GET /users/template => {:#?}", resp.status());
    }
}
