use crate::models::user::User;
use crate::schema::authenticable_users;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::error::CustomError;
use crate::services::jwt::{generate, Claims};
use bcrypt;

#[derive(Queryable, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct AuthenticableUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl AuthenticableUser {
    pub fn authenticate(
        connection: &PgConnection,
        form: AuthData
    ) -> Result<(AuthenticableUser, String), CustomError> {
        let user = Self::find_by_username(connection, &form.username)?;

        let valid = bcrypt::verify(&form.password, &user.password)?;
        if !valid {
            return Err(CustomError::AuthenticationError(String::from(
                "Invalid username or password",
            )));
        }

        let token = user.generate_jwt();

        Ok((user, token))
    }

    ///Create new authenticable user
    // pub fn new(username: String, hashed_password: String) -> NewAuthenticableUser {
    //     NewAuthenticableUser {
    //         username,
    //         password: hashed_password,
    //     }
    // }

    /// Fetch all users from the database
    pub fn all(connection: &PgConnection) -> Result<Vec<Self>, CustomError> {
        authenticable_users::table
            .load::<Self>(connection)
            .map_err(CustomError::from)
    }

    /// Fetch single user by username
    pub fn find_by_username(
        connection: &PgConnection,
        uname: &str,
    ) -> Result<AuthenticableUser, CustomError> {
        use crate::schema::authenticable_users::dsl::*;

        let user = authenticable_users
            .filter(username.eq(uname))
            .limit(1)
            .get_result(connection)?;
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
            password: String::new(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "authenticable_users"]
pub struct NewAuthenticableUser {
    pub username: String,
    pub password: String,
}

impl NewAuthenticableUser {
    /// Create a new user with username and password
    /// Password will be automatically hashed
    pub fn create(
        connection: &PgConnection,
        form: AuthData
    ) -> Result<AuthenticableUser, CustomError> {
        let hashed_password = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)?;
        //  {
        //     Ok(hashed) => hashed,
        //     Err(e) => {
        //         println!("Hashing password error: {}", e);
        //         return Err(CustomError::BcryptError(e));
        //     }
        // };
        match AuthenticableUser::find_by_username(connection, &form.username) {
            Ok(_user) => {
                return Err(CustomError::AuthenticationError(String::from(
                    "User already exists",
                )));
            }
            _ => (),
        }
        match User::find_by_username(connection, &form.username) {
            Ok(_user) => (),
            Err(_e) => {
                return Err(CustomError::AuthenticationError(String::from(
                    "User does not exists in slack workspace",
                )));
            }
        }
        let values = Self {
            username: String::from(&form.username),
            password: hashed_password.to_string(),
        };
        diesel::insert_into(authenticable_users::table)
            .values(&values)
            .get_result(connection)
            .map_err(CustomError::from)
    }
}

/// A struct containing data we need for auth
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}