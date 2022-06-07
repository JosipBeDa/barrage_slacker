use actix_cors::Cors;
use actix_web::{http, web};
use crate::routes::{self, chat, conversations, users};

/// Returns the routing config
pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    // GET /
    cfg.service(web::resource("/").route(web::get().to(routes::hello_world)));
    // GET /conversations
    cfg.service(
        web::resource("/conversations")
            .route(web::get().to(conversations::list_conversations::handler)),
    );
    // GET /conversations/{channel_id}
    cfg.service(
        web::resource("/conversations/{channel_id}")
            .route(web::get().to(conversations::get_conversation::handler)),
    );
    // POST /send_message
    cfg.service(web::resource("/send_message").route(web::post().to(chat::send_message::handler)));
    // GET /users
    cfg.service(web::resource("/users").route(web::get().to(users::list_users::handler)));
}

/// Return cors configuration for the project
pub fn setup_cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
