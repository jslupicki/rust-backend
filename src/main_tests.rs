use actix_http::cookie::Cookie;
use actix_http::http::StatusCode;
use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App};
use bytes::Bytes;

use rest::LoginDTO;

use crate::commons_for_tests;
use crate::test_data::URLS;

#[actix_rt::test]
async fn call_to_index_should_return_hello_world() {
    setup_test!("call_to_index_should_return_hello_world");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    let result = test::read_body(resp).await;
    assert_eq!(result, Bytes::from_static(b"Hello world"));
}

#[actix_rt::test]
async fn login_with_correct_credentials() {
    setup_test!("login_with_correct_credentials");

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
    setup_test!("login_with_incorrect_credentials");

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
    setup_test!("check_access_control");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let admin_session = login_as_admin(&mut app).await;
    let user_session = login_as_user(&mut app).await;

    assert!(user_session.is_some());
    assert!(admin_session.is_some());
    if let (Some(admin_session), Some(user_session)) = (admin_session, user_session) {
        info!("Got admin_session: {}", admin_session);
        info!("Got user_session: {}", user_session);
        for url in &*URLS {
            debug!("Checking {:#?}", url);
            let req_without_session = test::TestRequest::with_uri(url.url)
                .method(url.method.clone())
                .to_request();
            let req_with_admin_session = test::TestRequest::with_uri(url.url)
                .method(url.method.clone())
                .cookie(admin_session.clone())
                .to_request();
            let req_with_user_session = test::TestRequest::with_uri(url.url)
                .method(url.method.clone())
                .cookie(user_session.clone())
                .to_request();
            let resp_without_session = test::call_service(&mut app, req_without_session).await;
            let resp_with_admin_session =
                test::call_service(&mut app, req_with_admin_session).await;
            let resp_with_user_session = test::call_service(&mut app, req_with_user_session).await;
            if url.guarded {
                assert_eq!(
                    StatusCode::UNAUTHORIZED,
                    resp_without_session.status(),
                    "Call {:#?} without session should respond with UNAUTHORIZED",
                    url
                );
                if url.have_to_be_admin {
                    assert_ne!(
                        StatusCode::UNAUTHORIZED,
                        resp_with_admin_session.status(),
                        "Call {:#?} with admin session should NOT respond with UNAUTHORIZED",
                        url
                    );
                    assert_eq!(
                        StatusCode::UNAUTHORIZED,
                        resp_with_user_session.status(),
                        "Call {:#?} with user session should respond with UNAUTHORIZED",
                        url
                    );
                } else {
                    assert_ne!(
                        StatusCode::UNAUTHORIZED,
                        resp_with_admin_session.status(),
                        "Call {:#?} with admin session should NOT respond with UNAUTHORIZED",
                        url
                    );
                    assert_ne!(
                        StatusCode::UNAUTHORIZED,
                        resp_with_user_session.status(),
                        "Call {:#?} with user session should NOT respond with UNAUTHORIZED",
                        url
                    );
                }
            } else {
                assert_ne!(
                    StatusCode::UNAUTHORIZED,
                    resp_without_session.status(),
                    "Call {:#?} without session should NOT respond with UNAUTHORIZED",
                    url
                );
                assert_ne!(
                    StatusCode::UNAUTHORIZED,
                    resp_with_admin_session.status(),
                    "Call {:#?} with session should NOT respond with UNAUTHORIZED",
                    url
                );
            }
        }
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
        app,
    )
    .await
}

pub async fn login_as_user<S, B, E>(app: &mut S) -> Option<Cookie<'static>>
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    login(
        "user",
        "8ac76453d769d4fd14b3f41ad4933f9bd64321972cd002de9b847e117435b08b",
        app,
    )
    .await
}
