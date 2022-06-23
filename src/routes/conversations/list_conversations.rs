use crate::state::app::AppState;
use actix_web::web;
use crate::models::slack_responses::ChannelsList;
use crate::error::{process_typed, CustomError};

pub async fn handler(
    state: web::Data<AppState>,
) -> Result<web::Json<ChannelsList>, CustomError> {
    let res = state
        .client
        .get("https://slack.com/api/conversations.list")
        .send()
        .await?;
    let channels = process_typed(res).await?;
    Ok(web::Json(channels))
}
