use chrono::prelude::*;
use jsonwebtoken;
use crate::models::authenticable_users::AuthenticableUser;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
  pub sub: String,
  pub username: String,
  pub exp: i64,
  pub iat: i64,
}

/// Generate JWT for passed User
pub fn generate(user: &AuthenticableUser) -> String {
  let secret = match dotenv::var("JWT_SECRET") {
    Ok(s) => s,
    Err(_) => "".to_string(),
  };

  let duration = match dotenv::var("JWT_LIFETIME_IN_SECONDS") {
    Ok(d) => d,
    Err(_) => "300".to_string(),
  };

  let duration: i64 = duration.parse().unwrap();
  let exp = Utc::now() + chrono::Duration::seconds(duration);

  let claims = Claims {
    sub: String::from(&user.id.to_string()),
    username: String::from(&user.username),
    exp: exp.timestamp(),
    iat: Utc::now().timestamp(),
  };

  jsonwebtoken::encode(
    &jsonwebtoken::Header::default(),
    &claims,
    &jsonwebtoken::EncodingKey::from_secret(&secret.as_bytes()),
  )
  .unwrap_or_default()
}

/// Verify given token and return user if its okay
pub fn verify(token: String) -> Result<AuthenticableUser, jsonwebtoken::errors::Error> {
  let secret = match dotenv::var("JWT_SECRET") {
    Ok(s) => s,
    Err(_) => "".to_string(),
  };

  let token_data = jsonwebtoken::decode::<Claims>(
    &token,
    &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
    &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
  )?;

  Ok(AuthenticableUser::from_jwt(token_data.claims))
}