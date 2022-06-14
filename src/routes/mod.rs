pub mod conversations;
pub mod chat;
pub mod users;
pub mod auth;

use actix_web::{HttpResponse, Responder};
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello wordl")
}