use crate::state::app::AppState;
use actix_web::web;
use barrage_slacker::{process_response, CustomError, FormData};
use serde_json::Value;

///Sends a message to the specified channel
pub async fn handler(
    form: web::Form<FormData>,
    state: web::Data<AppState>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
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
        .await;

    let json = process_response(res).await;

    //Testing to see if we can obtain certain fields from the response
    match json {
        Ok(value) => {
            println!(
                "response {}, {}",
                value["channel"], value["message"]["username"]
            );
            Ok(value)
        }
        Err(e) => Err(e),
    }
}
