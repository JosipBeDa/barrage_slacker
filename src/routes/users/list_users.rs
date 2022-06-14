use crate::state::app::AppState;
use actix_web::web;
use crate::models::slack_responses::UsersList;
use crate::error::{process_typed, CustomError};

/// List all users
pub async fn handler(state: web::Data<AppState>) -> Result<web::Json<UsersList>, CustomError> {
    //Contact slack api
    let res = state
        .client
        .get("https://slack.com/api/users.list")
        .send()
        .await?;
    let users = process_typed(res).await?;
    Ok(web::Json(users))
}
