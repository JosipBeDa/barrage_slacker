#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub slack_id: String,
    pub slack_uname: String
}