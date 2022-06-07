pub mod conversations;
pub mod chat;
pub mod users;

use actix_web::{HttpResponse, Responder};
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello wordl")
}