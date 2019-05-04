use schema::users;

#[derive(Queryable, AsChangeset, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}
