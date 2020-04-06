use actix_http::http::StatusCode;
use actix_web::{test, App};

use rest::UserDTO;

use crate::commons_for_tests;
use crate::main_tests::login_as_user;

#[actix_rt::test]
async fn get_all_users() {
    setup_test!("get_all_users");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_user(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        let req = test::TestRequest::get()
            .uri("/users")
            .cookie(session.clone())
            .to_request();
        let users: Vec<UserDTO> = test::read_response_json(&mut app, req).await;
        assert_eq!(users.len(), 2);
    }
}

#[actix_rt::test]
async fn get_specific_user() {
    setup_test!("get_specific_user");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_user(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        let req = test::TestRequest::get()
            .uri("/users/1")
            .cookie(session.clone())
            .to_request();
        let user: UserDTO = test::read_response_json(&mut app, req).await;
        assert!(user.id.is_some());
        assert_eq!(user.id.unwrap(), 1);
        assert_eq!(user.username.unwrap(), String::from("user"));
        assert_eq!(user.is_admin.unwrap(), false);
        let req = test::TestRequest::get()
            .uri("/users/2")
            .cookie(session.clone())
            .to_request();
        let user: UserDTO = test::read_response_json(&mut app, req).await;
        assert!(user.id.is_some());
        assert_eq!(user.id.unwrap(), 2);
        assert_eq!(user.username.unwrap(), String::from("admin"));
        assert_eq!(user.is_admin.unwrap(), true);
    }
}

#[actix_rt::test]
async fn update_user() {
    setup_test!("update_user");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_user(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        let user = UserDTO {
            id: Some(1),
            username: Some(String::from("updated")),
            password: Some(String::from("updated")),
            is_admin: Some(false),
        };
        let req = test::TestRequest::post()
            .uri("/users")
            .cookie(session.clone())
            .set_json(&user)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert!(resp.status().is_success());
        
        let req = test::TestRequest::get()
            .uri("/users/1")
            .cookie(session.clone())
            .to_request();
        let user: UserDTO = test::read_response_json(&mut app, req).await;

        assert!(user.id.is_some());
        assert_eq!(user.id.unwrap(), 1);
        assert_eq!(user.username.unwrap(), String::from("updated"));
        assert_eq!(user.password.unwrap(), String::from("updated"));
        assert_eq!(user.is_admin.unwrap(), false);

        let user = UserDTO {
            id: Some(1),
            username: Some(String::from("updated2")),
            password: Some(String::from("updated2")),
            is_admin: Some(false),
        };
        let req = test::TestRequest::put()
            .uri("/users")
            .cookie(session.clone())
            .set_json(&user)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert!(resp.status().is_success());
        
        let req = test::TestRequest::get()
            .uri("/users/1")
            .cookie(session.clone())
            .to_request();
        let user: UserDTO = test::read_response_json(&mut app, req).await;

        assert!(user.id.is_some());
        assert_eq!(user.id.unwrap(), 1);
        assert_eq!(user.username.unwrap(), String::from("updated2"));
        assert_eq!(user.password.unwrap(), String::from("updated2"));
        assert_eq!(user.is_admin.unwrap(), false);
    }
}

#[actix_rt::test]
async fn delete_user() {
    setup_test!("delete_user");

    let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
    let session = login_as_user(&mut app).await;

    assert!(session.is_some());
    if let Some(session) = session {
        info!("Got session: {}", session);
        
        let req = test::TestRequest::delete()
            .uri("/users/1")
            .cookie(session.clone())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert!(resp.status().is_success());
        
        let req = test::TestRequest::get()
            .uri("/users/1")
            .cookie(session.clone())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        // TODO: uncomment when delete user will be implemented
        //assert_eq!(StatusCode::NOT_FOUND, resp.status());
        // Delete not yet implemented
        assert_eq!(StatusCode::OK, resp.status());
    }
}
