use actix_cors::Cors;
use actix_web::{get, post};
use actix_web::{http, web, App, HttpServer, ResponseError};
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug)]
struct FormData {
    channel: String,
    text: String,
    username: Option<String>,
    local_date_time: Option<String>,
    post_at: Option<i64>
}

#[derive(Debug)]
enum CustomError {
    SlackResponseError,
    BodyExtractionError,
    ConversionError,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::SlackResponseError => {
                write!(f, "There was an error in handling the response from Slack")
            }
            CustomError::BodyExtractionError => write!(f, "Unable to extract response body"),
            CustomError::ConversionError => write!(f, "Unable to convert body to json"),
        }
    }
}

impl ResponseError for CustomError {}

///Helper for processing requests sent to slack
async fn process_response(
    response: reqwest::Result<reqwest::Response>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    //Check the response and return if it errors
    if let Err(e) = response {
        println!("{}", e);
        return Err(CustomError::SlackResponseError);
    }
    //Get the body of the response
    if let Ok(body) = response.unwrap().text().await {
        //Try to convert it to json
        match serde_json::from_str(&body) {
            //If all went well return the json
            Ok(json) => Ok(web::Json(json)),
            //Otherwise return a conversion error
            Err(e) => {
                println!("Conversion error: {}", e);
                return Err(CustomError::ConversionError);
            }
        }
    } else {
        Err(CustomError::BodyExtractionError)
    }
}

/*****************************************HANDLERS***************************/

///Route that sends a message to the slack api
#[post("/send-message")]
async fn send_message(
    form: web::Form<FormData>,
    client: web::Data<reqwest::Client>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    println!("form: {:?}", form);

    //Make a form of the stuff we need to send in a form to slack
    let form = FormData {
        channel: form.channel.clone(),
        text: form.text.clone(),
        username: form.username.clone(),
        local_date_time: None,
        post_at: None
    };

    let res = client
        .post("https://slack.com/api/chat.postMessage")
        .form(&form)
        .send()
        .await;
    process_response(res).await
}

//Route that schedules a message to be sent to the slack api
#[post("/send-scheduled-message")]
async fn send_scheduled_message(
    form: web::Form<FormData>,
    client: web::Data<reqwest::Client>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    println!("form: {:?}", form.text);
    // Parses a DateTime object from a string
    let date_str = form.local_date_time.as_deref().unwrap();


    let date_time = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z").unwrap();
    // Converts local DateTime to a UTC timestamp 
    let timestamp = date_time.with_timezone(&Utc).timestamp();

    let body = FormData {
        channel: String::from(form.channel.clone()),
        text: String::from(form.text.clone()),
        username: None,
        post_at: Some(timestamp),
        local_date_time: None
    };

    println!("form: {:?}", body);

    let res = client
        .post("https://slack.com/api/chat.scheduleMessage")
        .form(&body)
        .send()
        .await;

    process_response(res).await
}

#[get("/users")]
async fn get_users(
    client: web::Data<reqwest::Client>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    //Contact slack api
    let res = client.get("https://slack.com/api/users.list").send().await;
    //Check the response and return if it errors
    if let Err(e) = res {
        println!("{}", e);
        return Err(CustomError::SlackResponseError);
    }
    process_response(res).await
}

#[get("/conversations")]
async fn get_conversations(
    client: web::Data<reqwest::Client>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    let res = client.get("https://slack.com/api/conversations.list").send().await;
    if let Err(e) = res {
        println!("{}", e);
        return Err(CustomError::SlackResponseError);
    }
    process_response(res).await
}

#[get("/conversations/{channel_id}")]
async fn get_conversation_info(
    path: web::Path<String>,
    client: web::Data<reqwest::Client>,
) -> actix_web::Result<web::Json<Value>, CustomError> {
    let res = client
        .get(format!(
            "https://slack.com/api/conversations.info?channel={}",
            path.into_inner()
        ))
        .send()
        .await;
    process_response(res).await
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Set the headers for the client builder, will need to be overriden if they mismatch
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(
        "Authorization",
        dotenv::var("BOT_TOKEN")
            .unwrap_or(String::new())
            .parse()
            .unwrap(),
    );

    //Build the client and wrap it in web::Data so we can access it from multiple threads
    let client_builder = reqwest::ClientBuilder::new().default_headers(headers);
    let client = web::Data::new(client_builder.build().unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(setup_cors())
            .app_data(client.clone())
            .service(get_conversation_info)
            .service(get_users)
            .service(send_message)
            .service(get_conversations)
            .service(send_scheduled_message)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Return cors configuration for the project
fn setup_cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
