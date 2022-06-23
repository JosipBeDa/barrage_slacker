use crate::error::{process_typed, CustomError};
use crate::models::message::{FormData, NewMessage};
use crate::models::channel::{Channel};
use crate::models::slack_responses::MessageSent;
use crate::state::app::AppState;
use crate::routes::conversations;
use actix_web::web;

///Sends a message to the specified channel
pub async fn handler(
    form: web::Form<FormData>,
    state: web::Data<AppState>,
) -> actix_web::Result<web::Json<MessageSent>, CustomError> {
    println!("form: {:?}", form);

    let form = FormData {
        channel: form.channel.clone(),
        text: form.text.clone(),
        username: form.username.clone(),
    };

    let res = state
        .client
        .post("https://slack.com/api/chat.postMessage")
        .form(&form)
        .send()
        .await?;

    let message_data: MessageSent = process_typed(res).await?;

    let connection = state.db_pool.get().expect("Couldn't get pool conn");

    let api_slack_channels = conversations::list_conversations::handler(state).await?;

    Channel::sync_api_channels(&connection, api_slack_channels.channels.clone())?;

    NewMessage::create(&connection, form)?;

    Ok(web::Json(message_data))
}
