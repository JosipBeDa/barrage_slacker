use crate::state::app::AppState;
use actix_web::web;
use crate::models::slack_responses::MessageSent;
use crate::error::{CustomError, process_typed};
use crate::diesel_functions::{store_message, FormData};


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
    store_message(
        &connection,
        &message_data.message.bot_id,
        &message_data.message.text,
        &message_data.channel,
    );
    Ok(web::Json(message_data))
}
