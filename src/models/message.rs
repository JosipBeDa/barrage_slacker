use crate::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub sender: String,
    pub body: String,
    pub channel: String,
    pub time_sent: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub sender: &'a str,
    pub body: &'a str,
    pub channel: &'a str,
    pub time_sent: chrono::DateTime<chrono::Utc>,
}
