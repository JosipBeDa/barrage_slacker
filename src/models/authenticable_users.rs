use crate::schema::authenticable_users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct AuthenticableUser {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="authenticable_users"]
pub struct NewAuthenticableUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}