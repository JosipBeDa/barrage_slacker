use actix_web::{web, ResponseError};
use r2d2;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Display;

/// A custom implementation for error handling that wraps all the possible errors we can come across
/// in the process of sending requests to Slack and converting response bodies to json.
#[derive(Debug)]
pub enum CustomError {
    DieselError(diesel::result::Error),
    BcryptError(bcrypt::BcryptError),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    SlackJsonError(serde_json::Value),
    AuthenticationError(String),
    R2D2Error(r2d2::Error),
    Base64Error(base64::DecodeError),
    UTF8Error(std::str::Utf8Error),
    JWTError(jsonwebtoken::errors::Error)
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ReqwestError(e) => {
                write!(f, "Reqwest error: {}", e)
            }
            CustomError::SlackJsonError(e) => write!(f, "Slack JSON error: {}", e),
            CustomError::SerdeError(e) => write!(f, "Serde error: {}", e),
            CustomError::DieselError(e) => write!(f, "Diesel error: {}", e),
            CustomError::BcryptError(e) => write!(f, "Bcrypt error: {}", e),
            CustomError::AuthenticationError(e) => write!(f, "Authentication error: {}", e),
            CustomError::R2D2Error(e) => write!(f, "r2d2 error: {}", e),
            CustomError::Base64Error(e) => write!(f, "base64 error: {}", e),
            CustomError::UTF8Error(e) => write!(f, "Utf8 error: {}", e),
            CustomError::JWTError(e) => write!(f, "JWT error: {}", e)
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
impl From<diesel::result::Error> for CustomError {
    fn from(error: diesel::result::Error) -> Self {
        CustomError::DieselError(error)
    }
}
impl From<bcrypt::BcryptError> for CustomError {
    fn from(error: bcrypt::BcryptError) -> Self {
        CustomError::BcryptError(error)
    }
}
impl From<r2d2::Error> for CustomError {
    fn from(error: r2d2::Error) -> Self {
        CustomError::R2D2Error(error)
    }
}
impl From<base64::DecodeError> for CustomError {
    fn from(error: base64::DecodeError) -> Self {
        CustomError::Base64Error(error)
    }
}
impl From<std::str::Utf8Error> for CustomError {
    fn from(error: std::str::Utf8Error) -> Self {
        CustomError::UTF8Error(error)
    }
}
impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        CustomError::JWTError(error)
    }
}

/// Helper for processing requests sent to slack. Slack sends a 200 OK response even when invalid data was sent,
/// so we have to check the "ok" field of the json response if we want to handle it properly. Once that's done we try
/// to extract the json from the body, if we fail serde throws an error. Finally, if all went well, we send the result back
/// to the actual handler.
pub async fn process_untyped(response: reqwest::Response) -> Result<web::Json<Value>, CustomError> {
    let json: Value = response.json().await?;
    if json["ok"] == false {
        return Err(CustomError::SlackJsonError(json));
    }
    Ok(web::Json(json))
}

// Since there's no way to check the 'ok' field of a typed response without it actually being true, serde will throw
// an error if
pub async fn process_typed<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, CustomError> {
    let json: T = response.json::<T>().await?;
    Ok(json)
}
