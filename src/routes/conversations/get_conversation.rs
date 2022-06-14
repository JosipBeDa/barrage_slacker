use crate::state::app::AppState;
use actix_web::web;
use crate::models::slack_responses::SingleChannel;
use crate::error::{process_typed, CustomError};

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
