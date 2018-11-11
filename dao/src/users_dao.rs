use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::{NewUser, User};
use sha3::{Digest, Sha3_256};
use users::dsl::*;

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
