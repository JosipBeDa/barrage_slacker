use crate::state::app::AppState;
use actix_web::web;
use barrage_slacker::models::slack_responses::ChannelsList;
use barrage_slacker::{process_typed, CustomError};

pub async fn handler(
    state: web::Data<AppState>,
) -> Result<web::Json<ChannelsList> /*<Value>*/, CustomError> {
    let res = state
        .client
        .get("https://slack.com/api/conversations.list")
        .send()
        .await?;
    let channels = process_typed(res).await?;
    Ok(web::Json(channels))
}
