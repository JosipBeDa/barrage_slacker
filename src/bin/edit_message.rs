extern crate diesel;
extern crate rust_project;

use self::diesel::prelude::*;
use self::models::Message;
use self::rust_project::*;
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    use rust_project::schema::messages::dsl::{body, messages};

    let id = args()
        .nth(1)
        .expect("edit_message requires a message id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let mut message_new = String::new();
    println!(
        "Editing message with id {}. What would you like your new message to be? When you're done press {} to exit",
        id, EOF
    );
    stdin().read_to_string(&mut message_new).unwrap();
    message_new = message_new.trim().to_string();
    diesel::update(messages.find(id))
        .set(body.eq(&message_new))
        .get_result::<Message>(&connection)
        .expect(&format!("Unable to find message {}", id));
    println!("Edited message {}", message_new);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";