#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub slack_id: String,
    pub slack_uname: String,
}

use crate::schema::users;
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub slack_id: &'a str,
    pub slack_uname: &'a str,
}
