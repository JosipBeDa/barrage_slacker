extern crate rust_project;
extern crate diesel;

use self::rust_project::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Who would you like your sender to be?");
    let mut sender = String::new();
    stdin().read_line(&mut sender).unwrap();
    let sender = &sender.trim(); // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", sender, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let message = create_message(&connection, sender, &body);
    println!("\nSaved message {} with id {}", sender, message.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";