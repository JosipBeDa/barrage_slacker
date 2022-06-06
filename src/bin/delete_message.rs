extern crate rust_project;
extern crate diesel;

use self::diesel::prelude::*;
use self::rust_project::*;
use std::env::args;

fn main() {
    use rust_project::schema::messages::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(messages.filter(sender.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} messages", num_deleted);
}