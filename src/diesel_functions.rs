use serde::{Deserialize, Serialize};
use crate::error::{CustomError};
extern crate dotenv;
use diesel::{PgConnection, RunQueryDsl};

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


/// Diesel functions // TO DO: Seperate this functionality to a seperate file
use crate::models::message::{Message, NewMessage};
use crate::models::authenticable_users::{AuthenticableUser, NewAuthenticableUser};
use crate::models::user::{User, NewUser};

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
    use crate::schema::authenticable_users;

    let new_user = NewAuthenticableUser {
        username: String::from(username),
        password: String::from(password)
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
