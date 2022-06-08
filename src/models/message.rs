use crate::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub body: String,
    pub channel: String,
    pub time_sent: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub id: &'a str,
    pub sender: &'a str,
    pub body: &'a str,
    pub channel: &'a str,
    pub time_sent: chrono::DateTime<chrono::Utc>,
}
