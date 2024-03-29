use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;

pub fn create_user(new_user: &NewUser, conn: &SqliteConnection) -> QueryResult<User> {
    conn.transaction(|| {
        insert_into(users)
            .values(new_user)
            .execute(conn)
            .and_then(|_| users.order(id.desc()).first(conn))
    })
}

pub fn update_user(user: &User, conn: &SqliteConnection) -> QueryResult<User> {
    conn.transaction(|| {
        diesel::update(users.filter(id.eq(user.id)))
            .set(user)
            .execute(conn)
            .and_then(|_| users.filter(id.eq(user.id)).first(conn))
    })
}

pub fn delete_user(user: &User, conn: &SqliteConnection) -> QueryResult<usize> {
    conn.transaction(|| diesel::delete(users.filter(id.eq(user.id))).execute(conn))
}

pub fn get_users(conn: &SqliteConnection) -> Vec<User> {
    users.load::<User>(conn).expect("Load users failed")
}

pub fn get_user(id_to_find: i32, conn: &SqliteConnection) -> Option<User> {
    users
        .filter(id.eq(id_to_find))
        .first(conn)
        .optional()
        .unwrap_or(None)
}

pub fn validate_user(
    username_p: &String,
    password_p: &String,
    conn: &SqliteConnection,
) -> Option<User> {
    info!(
        "Validate user '{}' with password '{}'",
        username_p, password_p
    );
    users
        .filter(username.eq(username_p).and(password.eq(password_p)))
        .first(conn)
        .optional()
        .unwrap_or(None)
}

#[cfg(test)]
mod tests {
    use diesel::result::DatabaseErrorKind::UniqueViolation;
    use diesel::result::Error::DatabaseError;
    use sha3::{Digest, Sha3_256};

    use crate::common_for_tests::*;

    use super::*;

    #[test]
    fn crud_operations_on_user() {
        let conn = &initialize();

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
    }

    #[test]
    fn check_hash() {
        initialize_log();
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
        let conn = &initialize();

        let initially_user_count = user_count(conn);
        let all_users = get_users(conn);
        assert_eq!(all_users.len() as i64, initially_user_count)
    }

    #[test]
    fn check_validate_user() {
        let conn = &initialize();

        assert!(validate_user(
            &"admin".to_string(),
            &"fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b".to_string(),
            conn,
        )
        .is_some());
        assert!(validate_user(
            &"wrong".to_string(),
            &"fb001dfcffd1c899f3297871406242f097aecf1a5342ccf3ebcd116146188e4b".to_string(),
            conn,
        )
        .is_none());
        assert!(validate_user(&"admin".to_string(), &"wrong".to_string(), conn).is_none());
    }

    #[test]
    fn should_prevent_creating_users_with_the_same_username() {
        let conn = &initialize();

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
            _ => panic!(
                "Should report: UNIQUE constraint failed: users.username and instead I got {:?}",
                rows_inserted
            ),
        }
    }

    #[test]
    fn check_get_user() {
        let conn = &initialize();

        let user_in_db = get_user(2, conn).unwrap();
        assert_eq!("admin", user_in_db.username);
        assert!(user_in_db.is_admin);
    }

    #[test]
    fn check_update_user() {
        let conn = &initialize();

        let mut admin_in_db = get_user(2, conn).unwrap();
        admin_in_db.password = "new_password".to_string();
        let updated_rows = update_user(&admin_in_db, conn);
        assert!(updated_rows.is_ok());
        let admin_in_db = get_user(2, conn).unwrap();
        assert_eq!("new_password".to_string(), admin_in_db.password);
        let updated_user = updated_rows.unwrap();
        assert_eq!(2, updated_user.id);
        assert_eq!("admin".to_string(), updated_user.username);
        assert_eq!("new_password".to_string(), updated_user.password);
        assert!(updated_user.is_admin);
    }

    #[test]
    fn create_user_should_return_created_user() {
        let conn = &initialize();

        let new_user = NewUser {
            username: "new_username".to_string(),
            password: "new_password".to_string(),
            is_admin: false,
        };
        let created_user = create_user(&new_user, conn).unwrap();
        assert_eq!(3, created_user.id);
        assert_eq!(new_user.username, created_user.username);
        assert_eq!(new_user.password, created_user.password);
        assert_eq!(new_user.is_admin, created_user.is_admin);
    }

    #[test]
    fn check_delete_user() {
        let conn = &initialize();

        let admin_in_db = get_user(2, conn).unwrap();
        let deleted_rows = delete_user(&admin_in_db, conn);
        assert_eq!(deleted_rows.unwrap(), 1);
        let admin_in_db = get_user(2, conn);
        assert!(admin_in_db.is_none());
    }

    fn hash(text: &String) -> String {
        format!("{:x}", Sha3_256::digest(text.as_bytes()))
    }
}
