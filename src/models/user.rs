#[derive(Queryable, Debug)]
pub struct User {
    pub slack_id: String,
    pub slack_uname: String,
}

use crate::schema::users;
#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser {
    pub slack_id: String,
    pub slack_uname: String,
}
