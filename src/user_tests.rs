use crate::main_tests::{MUTEX, initialize_log, setup_db, tear_down_db, login_as_user};
use actix_web::{test, App};
use rest::UserDTO;

#[actix_rt::test]
async fn get_all_users() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start get_all_users() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End get_all_users() test");
    }

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
        info!("Got {} users", users.len());
        assert_eq!(users.len(), 2);
        for user in users {
            debug!("User: {:#?}", user);
        }
    }
}

#[actix_rt::test]
async fn get_specific_user() {
    let lock = MUTEX.lock();
    initialize_log();
    info!("Start get_specific_user() test");
    setup_db();
    defer! {
        tear_down_db();
        info!("End get_specific_user() test");
    }

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
