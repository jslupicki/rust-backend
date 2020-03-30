extern crate dao;
#[cfg(test)]
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rest;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;
    use std::{thread, time};

    use actix_web::{test, App};
    use actix_web::dev::ServiceResponse;
    use bytes::Bytes;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations;

    use dao::{get_connection, initialize_db};
    use rest::LoginDTO;
    use actix_http::http::StatusCode;
    use actix_http::Request;
    use actix_service::{Service};

    use super::*;

    lazy_static! {
        static ref MUTEX: Mutex<i32> = Mutex::new(0i32);
    }

    #[actix_rt::test]
    async fn call_to_index_should_return_hello_world() {
        let lock = MUTEX.lock();
        initialize_log();
        info!("Start call_to_index_should_return_hello_world() test");
        setup_db();

        let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        tear_down_db();
        assert!(resp.status().is_success());
        let result = test::read_body(resp).await;
        assert_eq!(result, Bytes::from_static(b"Hello world"));
        info!("End call_to_index_should_return_hello_world() test");
    }

    #[actix_rt::test]
    async fn login_with_correct_credentials() {
        let lock = MUTEX.lock();
        initialize_log();
        info!("Start login_with_correct_credentials() test");
        setup_db();

        let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
        let credentials = LoginDTO {
            username: String::from("admin"),
            password: String::from(
                "fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b",
            ),
        };
        let req = test::TestRequest::post()
            .uri("/auth")
            .set_json(&credentials)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let session = resp.response().cookies().find(|c| c.name() == "session");

        tear_down_db();

        assert!(resp.status().is_success());
        assert!(session.is_some());
        info!("End login_with_correct_credentials() test");
    }

    #[actix_rt::test]
    async fn login_with_incorrect_credentials() {
        let lock = MUTEX.lock();
        initialize_log();
        info!("Start login_with_incorrect_credentials() test");
        setup_db();

        let mut app = test::init_service(App::new().configure(|cfg| rest::config_all(cfg))).await;
        let credentials = LoginDTO {
            username: String::from("admin"),
            password: String::from(
                "wrong password",
            ),
        };
        let req = test::TestRequest::post()
            .uri("/auth")
            .set_json(&credentials)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        let session = resp.response().cookies().find(|c| c.name() == "session");

        tear_down_db();

        assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
        assert!(!session.is_some());
        info!("End login_with_incorrect_credentials() test");
    }

    #[test]
    fn integration_test3() {
        let lock = MUTEX.lock();
        initialize_log();
        perform_test("Integration Test3");
    }

    fn initialize_log() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
    }

    #[allow(unused_variables)]
    fn perform_test(name: &str) {
        info!("Start {}", name);
        setup_db();
        let timeout = time::Duration::from_millis(200);
        for i in 1..10 {
            debug!("In {}: {}", name, i);
            thread::sleep(timeout);
        }
        tear_down_db();
        info!("End of {}", name);
    }

    fn setup_db() {
        info!("Initialize DB (if not exist), run migrations");
        env::set_var("DATABASE_URL", ":memory:");
        env::set_var("POOL_SIZE", "1");
        initialize_db().unwrap();
    }

    fn tear_down_db() {
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

    async fn login<S, B, E>(username: &str, password: &str, app: &mut S) -> Option<String> 
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
        let session = resp.response().cookies().find(|c| c.name() == "session");
        if let Some(session) = session {
            Some(String::from(session.value()))
        } else {
            None
        }
    }
}
