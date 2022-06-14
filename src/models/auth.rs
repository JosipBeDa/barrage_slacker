use super::authenticable_users::AuthenticableUser;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, PgConnection};
use crate::schema::authenticable_users;
// use std::{error::Error, fmt};
use crate::error::{CustomError};

#[derive(Queryable, PartialEq, Debug, serde::Deserialize)]
pub struct AuthenticableUserData {
  pub username: String,
  pub password: String,
}

impl AuthenticableUserData {
    pub fn authenticate<'b>(
        connection: &PgConnection,
        username: &'b str,
        password: &'b str,
    ) -> Result<(AuthenticableUser, String), CustomError> {
        let user = match authenticable_users::table
            .filter(authenticable_users::username.eq(&username))
            .load::<AuthenticableUser>(connection)
        {
            Ok(mut results) => match results.pop() {
                Some(item) => item,
                _ => {
                    println!("Authentication: No user found with username: {}", &username);
                    return Err(CustomError::ValidationError(String::from("No User found")));
                }
            },
            Err(e) => {
                println!(
                    "Authentication: Something went wrong with getting the user out of db: {:?}",
                    &e 
                );
                return Err(CustomError::DieselError(e))
            }
        };

        AuthenticableUserData::verify(String::from(password), &user)?;

        let token = user.generate_jwt();

        Ok((user, token))
    }

    /// Verify the bcrypt password
  fn verify(password: String, user: &AuthenticableUser) -> Result<(), CustomError> {
    match bcrypt::verify(&password, &user.password) {
      Ok(res) => {
        if res == true {
          Ok(())
        } else {
          println!("Authentication: bcrypt verify error for: {}", &user.username);
          Err(CustomError::ValidationError(String::from("Invalid username or Passowrd")))
        }
      }
      Err(e) => {
        println!(
          "Authentication: bcrypt verify error: {}, for: {}",
          e, &user.username
        );
        Err(CustomError::BcryptError(e))
      }
    }
  }
}