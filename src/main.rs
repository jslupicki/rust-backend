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
#[macro_use]
extern crate scopeguard;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Start application");

    dao::initialize_db().unwrap();
    rest::start().await
}

#[cfg(test)]
mod test_data;

#[cfg(test)]
mod tests {

    use std::env;
    use std::sync::Mutex;

    use actix_web::dev::ServiceResponse;
    use actix_web::{test, App};
    use bytes::Bytes;
    use diesel::sqlite::SqliteConnection;
    use diesel_migrations;

    use actix_http::http::{Cookie, StatusCode};
    use actix_http::{Request};
    use actix_service::Service;
    use dao::{get_connection, initialize_db};
    use rest::LoginDTO;
    use test_data::URLS;

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
                    assert_eq!(StatusCode::UNAUTHORIZED, resp_without_session.status(), 
                    "Call {:#?} without session should respond with UNAUTHORIZED", url);
                    assert_ne!(StatusCode::UNAUTHORIZED, resp_with_session.status(), 
                    "Call {:#?} with session should NOT respond with UNAUTHORIZED", url);
                } else {
                    assert_ne!(StatusCode::UNAUTHORIZED, resp_without_session.status(), 
                    "Call {:#?} without session should NOT respond with UNAUTHORIZED", url);
                    assert_ne!(StatusCode::UNAUTHORIZED, resp_with_session.status(), 
                    "Call {:#?} with session should NOT respond with UNAUTHORIZED", url);
                }
            }
        }
     }

    fn initialize_log() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
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

    async fn login<S, B, E>(username: &str, password: &str, app: &mut S) -> Option<Cookie<'static>>
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
}
