use actix_web::{web, ResponseError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

/// A struct containing data we need to send slack to post a message.
#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    pub channel: String,
    pub text: String,
    pub username: String,
}

/// A struct containing data we need for auth
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

/// A custom implementation for error handling that wraps all the possible errors we can come across
/// in the process of sending requests to Slack and converting response bodies to json.
#[derive(Debug)]
pub enum CustomError {
    DieselError(diesel::result::Error),
    BcryptError(bcrypt::BcryptError),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    SlackJsonError(serde_json::Value),
    ValidationError(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ReqwestError(e) => {
                write!(f, "Reqwest error: {}", e)
            }
            CustomError::SlackJsonError(e) => write!(f, "Slack JSON error: {}", e),
            CustomError::SerdeError(e) => write!(f, "Serde error: {}", e),
            CustomError::DieselError(e) => write!(f, "Diesel Error: {}", e),
            CustomError::BcryptError(e) => write!(f, "Bcrypt Error: {}", e),
            CustomError::ValidationError(e) => write!(f, "Validation Error: {}", e),
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

// Diesel functions // TO DO: Seperate this functionality to a seperate file
#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::{PgConnection, RunQueryDsl};
use models::message::{Message, NewMessage};
use models::authenticable_users::{AuthenticableUser, NewAuthenticableUser};
use models::user::{User, NewUser};

pub mod models;
pub mod schema;

pub fn store_message<'a>(
    conn: &PgConnection,
    sender: &'a str,
    body: &'a str,
    channel: &'a str,
) -> Message {
    use crate::schema::messages;

    let time_sent = chrono::offset::Utc::now();

    let new_message = NewMessage {
        sender,
        body,
        channel,
        time_sent,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving message!")
}

/// function for registering a new user to the database
pub fn register_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> Result<AuthenticableUser, CustomError> {
    use schema::authenticable_users;

    let new_user = NewAuthenticableUser {
        username,
        password
    };

    match fetch_auth_user_by_username(&conn, &username) {
        Ok(_user) => {
            return Err(CustomError::ValidationError(String::from("User already exists")));
        },
        _ => ()
    }

    match fetch_slack_user_by_username(&conn, &username) {
        Ok(_user) => (),
        Err(_e) => {
            return Err(CustomError::ValidationError(String::from("User does not exists in slack workspace")));
        }
    }


    diesel::insert_into(authenticable_users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(CustomError::from)
}

/// function for confirming user credentials and logging in
pub fn fetch_auth_user_by_username(conn: &PgConnection, slack_username: &str) -> Result<AuthenticableUser, CustomError> {
    use crate::schema::authenticable_users::dsl::*;
    use crate::diesel::{ExpressionMethods, QueryDsl};
    
    let user = authenticable_users.filter(username.eq(slack_username)).limit(1).get_result(conn)?;
    Ok(user)
}

/// function for fetching slack users from the database by their slack username
pub fn fetch_slack_user_by_username(conn: &PgConnection, slack_username: &str) -> Result<User, CustomError> {
    use crate::schema::users::dsl::*;
    use crate::diesel::{ExpressionMethods, QueryDsl};

    let res = users.filter(slack_uname.eq(slack_username)).limit(1).get_result(conn)?;

    Ok(res)
}

/// function for fetching all slack users stored in the database
pub fn fetch_slack_users(conn: &PgConnection) -> Result<Vec<User>, CustomError> {
    use crate::schema::users::dsl::*;

    let slack_users = users.load::<User>(conn)?;
    Ok(slack_users)
}

/// function for fetching all auth users stored in the database
pub fn fetch_authenticable_users(conn: &PgConnection) -> Result<Vec<AuthenticableUser>, CustomError> {
    use crate::schema::authenticable_users::dsl::*;

    let auth_users = authenticable_users.load::<AuthenticableUser>(conn)?;
    Ok(auth_users)
}

/// function for inserting multiple slack users in the database
pub fn insert_slack_users(conn: &PgConnection, slack_users: Vec<NewUser>) -> Result<Vec<User>, CustomError> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(slack_users)
        .get_results(conn)
        .map_err(CustomError::from)
}
