pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{Message, NewMessage};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_message<'a>(conn: &PgConnection, sender: &'a str, body: &'a str) -> Message {
    use schema::messages;
    let time_sent = chrono::offset::Utc::now();

    let new_message = NewMessage { sender, body, time_sent };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving message")
}
