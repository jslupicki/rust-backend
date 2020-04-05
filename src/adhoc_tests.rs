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
