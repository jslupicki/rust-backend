use std::io::stdout;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::RunMigrationsError;
use sha3::{Digest, Sha3_256};

use models::{NewUser, User};
use schema::users::dsl::*;

pub fn create_user(new_user: &NewUser, conn: &SqliteConnection) -> QueryResult<usize> {
    insert_into(users).values(new_user).execute(conn)
}

pub fn hash(text: &String) -> String {
    let mut h = Sha3_256::default();
    h.input(text.as_bytes());
    format!("{:x}", h.result())
}

pub fn get_users(conn: &SqliteConnection) -> Vec<User> {
    users.load::<User>(conn).expect("Load users failed")
}

pub fn validate_user(username_p: &String, password_p: &String, conn: &SqliteConnection) -> bool {
    info!(
        "Validate user '{}' with password '{}'",
        username_p, password_p
    );
    let how_many_users_fit: i64 = users
        .select(count(id))
        .filter(username.eq(username_p).and(password.eq(password_p)))
        .first(conn)
        .expect("Error validate user");
    how_many_users_fit > 0
}

embed_migrations!("./migrations");

pub fn initialize_db(conn: &SqliteConnection) -> Result<(), RunMigrationsError> {
    info!("Initialize DB (if not exist), run migrations");
    embedded_migrations::run_with_output(conn, &mut stdout())
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::io::stdout;
    use std::sync::Arc;

    use diesel;
    use diesel::result::DatabaseErrorKind::UniqueViolation;
    use diesel::result::Error::DatabaseError;
    use log4rs;
    use monitor::Monitor;

    use super::*;

    static TEST_DB_NAME: &str = "test_db.sqlite3";

    lazy_static! {
        static ref MON: Arc<Monitor<bool>> = Arc::new(Monitor::new(false));
    }

    embed_migrations!("./migrations");

    fn initialize_db() -> SqliteConnection {
        let _ = remove_file(TEST_DB_NAME); // Ignore error when there is no TEST_DB_NAME file
        let conn = SqliteConnection::establish(TEST_DB_NAME)
            .expect(&format!("Error connecting to {}", TEST_DB_NAME));
        embedded_migrations::run_with_output(&conn, &mut stdout()).unwrap();
        conn
    }

    fn user_count(conn: &SqliteConnection) -> i64 {
        users.select(count(id)).first(conn).unwrap()
    }

    fn assert_user_count(expected: i64, conn: &SqliteConnection) {
        let user_count = user_count(conn);
        assert_eq!(user_count, expected);
    }

    #[test]
    fn crud_operations_on_user() {
        MON.with_lock(|_| {
            // Initialize
            let _ = log4rs::init_file("log4rs.yml", Default::default());
            let conn = &initialize_db();

            let initially_user_count = user_count(conn);

            let test_user = "test_admin";
            let test_updated_user = "test_updated_admin";
            let test_pass = "test_admin_pass";

            // Insert new_user
            let new_user = NewUser {
                username: test_user.to_string(),
                password: test_pass.to_string(),
                is_admin: true,
            };
            let rows_inserted = insert_into(users).values(&new_user).execute(conn);
            assert_eq!(Ok(1), rows_inserted);
            assert_user_count(initially_user_count + 1, conn);

            // Read user
            let users_in_db = users
                .filter(username.eq(&new_user.username))
                .limit(2)
                .load::<User>(conn)
                .expect("Error loading users");
            assert_eq!(1, users_in_db.len());
            let user = &users_in_db[0];
            assert_eq!(&new_user.username, &user.username);
            assert_eq!(&new_user.password, &user.password);
            assert_eq!(&new_user.is_admin, &user.is_admin);

            // Update username to "new_admin"
            let rows_updated = diesel::update(users.filter(id.eq(user.id)))
                .set(username.eq(test_updated_user.to_string()))
                .execute(conn);
            assert_eq!(Ok(1), rows_updated);

            // Read updated_user and check if have changed username
            let updated_user = users.filter(id.eq(user.id)).first::<User>(conn).unwrap();
            assert_eq!(&test_updated_user.to_string(), &updated_user.username);
            assert_eq!(&new_user.password, &updated_user.password);
            assert_eq!(&new_user.is_admin, &updated_user.is_admin);

            // Delete user from DB and DB should be in initial state
            diesel::delete(users.filter(id.eq(user.id)))
                .execute(conn)
                .unwrap();
            assert_user_count(initially_user_count, conn);
        })
    }

    #[test]
    fn check_hash() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let text = String::from("text");
        let hash = hash(&text);
        info!("hash of '{}': {}", text, hash);
        assert_eq!(
            hash,
            "987b43dbd4b9c71bdc9f6262a80fdde5e5b6e095acadfbabfe4cafc8f34b419a"
        );
    }

    #[test]
    fn check_get_users() {
        MON.with_lock(|_| {
            // Initialize
            let _ = log4rs::init_file("log4rs.yml", Default::default());
            let conn = &initialize_db();
            let initially_user_count = user_count(conn);
            let all_users = get_users(conn);
            assert_eq!(all_users.len() as i64, initially_user_count)
        })
    }

    #[test]
    fn check_validate_user() {
        MON.with_lock(|_| {
            // Initialize
            let _ = log4rs::init_file("log4rs.yml", Default::default());
            let conn = &initialize_db();
            assert_eq!(
                true,
                validate_user(
                    &"admin".to_string(),
                    &"fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b".to_string(),
                    conn,
                )
            );
            assert_eq!(
                false,
                validate_user(
                    &"wrong".to_string(),
                    &"fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b".to_string(),
                    conn,
                )
            );
            assert_eq!(
                false,
                validate_user(&"admin".to_string(), &"wrong".to_string(), conn)
            );
        })
    }

    #[test]
    fn should_prevent_creating_users_with_the_same_username() {
        MON.with_lock(|_| {
            // Initialize
            let _ = log4rs::init_file("log4rs.yml", Default::default());
            let conn = &initialize_db();
            // Insert new_user
            let new_user = NewUser {
                username: "admin".to_string(),
                password: "not_important".to_string(),
                is_admin: true,
            };
            let rows_inserted = insert_into(users).values(&new_user).execute(conn);
            match rows_inserted {
                Err(DatabaseError(UniqueViolation, msg)) => {
                    assert_eq!(msg.message(), "UNIQUE constraint failed: users.username")
                }
                _ => assert!(
                    false,
                    format!("Should report: UNIQUE constraint failed: users.username and instead I got {:?}", rows_inserted)
                ),
            }
        })
    }
}
