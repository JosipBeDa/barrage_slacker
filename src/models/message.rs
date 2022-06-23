use crate::schema::messages;
use diesel::{PgConnection, RunQueryDsl};
use crate::error::CustomError;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::channel::Channel;

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
pub struct NewMessage {
    pub sender: String,
    pub body: String,
    pub channel: String,
    pub time_sent: chrono::DateTime<chrono::Utc>,
}

impl NewMessage {
    pub fn create(
        conn: &PgConnection,
        form: FormData
    ) -> Result<Message, CustomError> {
    
        let time_sent = chrono::offset::Utc::now();
        println!("MDA");
        let sender = User::find_by_username(conn, &form.username)?;
        println!("MDA2");
        let channel = Channel::find_by_channel_id(conn, form.channel)?;
        
        println!("MDA3");
        let new_message = NewMessage {
            body: form.text,
            channel: channel.id,
            sender: sender.slack_id,
            time_sent,
        };
    
        println!("MDA4");

        diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result(conn)
            .map_err(CustomError::from)
    }
}

/// A struct containing data we need to send slack to post a message.
#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    pub channel: String,
    pub text: String,
    pub username: String,
}