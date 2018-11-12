use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::{NewUser, User};
use schema::users::dsl::*;
use sha3::{Digest, Sha3_256};

pub fn insert_default_users(conn: &SqliteConnection) {
    let default_users = vec![
        NewUser {
            username: "admin".to_string(),
            password: "admin".to_string(),
            is_admin: true,
        },
        NewUser {
            username: "user".to_string(),
            password: "user".to_string(),
            is_admin: false,
        },
    ];
    for user in &default_users {
        if let Ok(is_admin_exist) =
            select(exists(users.filter(username.eq(&user.username)))).get_result::<bool>(conn)
        {
            if !is_admin_exist {
                let new_user = create_user(&user, conn);
                assert_eq!(new_user, Ok(1), "Adding '{}' user FAILED", &user.username);
            }
        }
    }
}

pub fn create_user(new_user: &NewUser, conn: &SqliteConnection) -> QueryResult<usize> {
    insert_into(users).values(new_user).execute(conn)
}

pub fn hash(text: &String) -> String {
    let mut h = Sha3_256::default();
    h.input(text.as_bytes());
    format!("{:x}", h.result())
}

#[cfg(test)]
mod testy {
    use super::*;
    use std::fs::remove_file;
    use std::io::stdout;
    use log4rs;
    use diesel;

    static TEST_DB_NAME: &str = "test_db.sqlite3";

    embed_migrations!("./migrations");

    fn initialize_db() -> SqliteConnection {
        let _ = remove_file(TEST_DB_NAME); // Ignore error when there is no TEST_DB_NAME file
        let conn = SqliteConnection::establish(TEST_DB_NAME)
            .expect(&format!("Error connecting to {}", TEST_DB_NAME));
        embedded_migrations::run_with_output(&conn, &mut stdout()).unwrap();
        conn
    }

    fn assert_user_count(expected: i64, conn: &SqliteConnection) {
        let user_count: i64 = users.select(count(id)).first(conn).unwrap();
        assert_eq!(user_count, expected);
    }

    #[test]
    fn crud_operations_on_user() {
        // Initialize
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = &initialize_db();

        // Check if DB is empty
        assert_user_count(0, conn);

        // Insert new_user
        let new_user = NewUser {
            username: "admin".to_string(),
            password: "admin_pass".to_string(),
            is_admin: true,
        };
        let rows_inserted = insert_into(users).values(&new_user).execute(conn);
        assert_eq!(Ok(1), rows_inserted);
        assert_user_count(1, conn);

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
            .set(username.eq("new_admin".to_string()))
            .execute(conn);
        assert_eq!(Ok(1), rows_updated);

        // Read updated_user and check if have changed username
        let updated_user = users.filter(id.eq(user.id)).first::<User>(conn).unwrap();
        assert_eq!(&"new_admin".to_string(), &updated_user.username);
        assert_eq!(&new_user.password, &updated_user.password);
        assert_eq!(&new_user.is_admin, &updated_user.is_admin);

        // Delete user from DB and DB should be empty
        diesel::delete(users.filter(id.eq(user.id)))
            .execute(conn)
            .unwrap();
        assert_user_count(0, conn);
    }

    #[test]
    fn check_insert_default_users() {
        // Initialize
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = &initialize_db();

        // Check if DB is empty
        assert_user_count(0, conn);

        insert_default_users(conn);
        assert_user_count(2, conn);
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
}
