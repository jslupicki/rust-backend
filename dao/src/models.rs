use serde::Serialize;

use schema::users;

#[derive(Queryable, AsChangeset, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}
