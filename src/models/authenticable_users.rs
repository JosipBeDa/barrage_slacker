use crate::schema::authenticable_users;
use serde::{Deserialize, Serialize};
// use diesel::{PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::prelude::*;
use crate::error::{CustomError};
use crate::services::jwt::{generate, Claims};

#[derive(Queryable, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AuthenticableUser {
    pub id: i32,
    pub username: String,
    pub password: String
}

impl AuthenticableUser {
    ///Create new authenticable user
    pub fn new(username: String, hashed_password: String) -> NewAuthenticableUser {
        NewAuthenticableUser {
            username,
            password: hashed_password
        }
    }

    /// Fetch all users from the database
    pub fn all(connection: &PgConnection) -> Result<Vec<Self>, CustomError> {
        authenticable_users::table.load::<Self>(connection).map_err(CustomError::from)
    }

    /// Fetch single user by username
    pub fn find_by_email(
        connection: &PgConnection,
        username: &str
    ) -> Result<AuthenticableUser, CustomError> {
        use crate::schema::authenticable_users::dsl::*;

        let user = authenticable_users
            .filter(username.eq(username)).limit(1).get_result(connection)?;
        Ok(user)
    }

    /// Genereta authentication JWT token
    pub fn generate_jwt(&self) -> String {
        generate(&self)
    }

    /// Convert decoded claims from JWT into an AuthenticableUser object
    pub fn from_jwt(claims: Claims) -> Self {
        AuthenticableUser {
            id: claims.sub.parse::<i32>().unwrap(),
            username: String::from(&claims.username),
            password: String::new()
        }
    }
}




#[derive(Insertable)]
#[table_name="authenticable_users"]
pub struct NewAuthenticableUser {
    pub username: String,
    pub password: String,
}