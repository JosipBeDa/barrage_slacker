use crate::state::app::AppState;
use actix_web::web;
use barrage_slacker::{process_response, CustomError};
use serde_json::Value;

pub async fn handler(state: web::Data<AppState>) -> Result<web::Json<Value>, CustomError> {
    let res = state
        .client
        .get("https://slack.com/api/conversations.list")
        .send()
        .await;
    process_response(res).await
}
