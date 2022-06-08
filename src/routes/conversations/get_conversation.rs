use crate::state::app::AppState;
use actix_web::web;
use barrage_slacker::models::slack_responses::SingleChannel;
use barrage_slacker::{process_typed, CustomError};

pub async fn handler(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<web::Json<SingleChannel>, CustomError> {
    let res = state
        .client
        .get(format!(
            "https://slack.com/api/conversations.info?channel={}",
            path.into_inner()
        ))
        .send()
        .await?;
    let channel = process_typed(res).await?;
    Ok(web::Json(channel))
}
