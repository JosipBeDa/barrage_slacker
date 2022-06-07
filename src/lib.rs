use actix_web::{web, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;

/// A struct containing data we need to send slack to post a message.
#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    pub channel: String,
    pub text: String,
    pub username: String,
}

/// A custom implementation for error handling that wraps all the possible errors we can come across
/// in the process of sending requests to Slack and converting response bodies to json.
#[derive(Debug)]
pub enum CustomError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    SlackJsonError(serde_json::Value),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ReqwestError(e) => {
                write!(f, "Reqwest error: {}", e)
            }
            CustomError::SlackJsonError(e) => write!(f, "Slack JSON error: {}", e),
            CustomError::SerdeError(e) => write!(f, "Serde error: {}", e),
        }
    }
}

// Must be implemented for us to be able to send them back as a response
impl ResponseError for CustomError {}
impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError::ReqwestError(error)
    }
}
impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::SerdeError(error)
    }
}

/// Helper for processing requests sent to slack. First we check if the request was sent successfully
/// and throw a ReqwestError if something goes wrong. Slack sends a 200 OK response even when invalid data was sent,
/// so we have to check the "ok" field of the json response if we want to handle it properly. Once that's done we try
/// to extract the json from the body, if we fail we throw a SerdeError. Finally, if all went well, we send the result back
/// to the actual handler.
pub async fn process_response(
    response: reqwest::Result<reqwest::Response>,
) -> Result<web::Json<Value>, CustomError> {
    match response {
        Ok(res) => {
            let json: Value = res.json().await?;
            if json["ok"] == false {
                return Err(CustomError::SlackJsonError(json));
            }
            Ok(web::Json(json))
        }
        Err(error) => Err(CustomError::ReqwestError(error)),
    }
}
