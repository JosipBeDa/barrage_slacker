use crate::error::CustomError;
use crate::models::authenticable_users::{AuthenticableUser, AuthData};
use crate::state::app::AppState;
use actix_web::web;

pub async fn handler(
    form: web::Form<AuthData>,
    state: web::Data<AppState>,
) -> actix_web::Result<web::Json<(AuthenticableUser, String)>, CustomError> {
    println!("form: {:?}", form);

    let connection = state.db_pool.get().expect("Couldn't get pool conn");

    let form = AuthData {
        username: form.username.clone(),
        password: form.password.clone(),
    };

    let result = AuthenticableUser::authenticate(&connection, form)?;

    Ok(web::Json(result))
}
