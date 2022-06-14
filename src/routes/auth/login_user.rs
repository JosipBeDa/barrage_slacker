use crate::state::app::AppState;
use actix_web::web;
use crate::error::{CustomError};
use crate::models::authenticable_users::AuthenticableUser;
use crate::diesel_functions::{fetch_auth_user_by_username, AuthData};
use bcrypt::{verify};

pub async fn handler (
    form: web::Form<AuthData>,
    state: web::Data<AppState>
) -> actix_web::Result<web::Json<AuthenticableUser>, CustomError> {
    println!("form: {:?}", form);

    let connection = state.db_pool.get().expect("Couldn't get pool conn");

    let form = AuthData {
        username: form.username.clone(),
        password: form.password.clone(),
    };

    let user = fetch_auth_user_by_username(&connection, &form.username)?;

    let valid = verify(&form.password, &user.password)?;

    if !valid {
        return Err(CustomError::ValidationError(String::from("Invalid username or password")));
    }

    Ok(web::Json(user))
}
