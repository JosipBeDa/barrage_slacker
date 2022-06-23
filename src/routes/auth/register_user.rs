use crate::error::CustomError;
use crate::models::authenticable_users::{AuthenticableUser, NewAuthenticableUser, AuthData};
use crate::models::user::User;
use crate::routes::users;
use crate::state::app::AppState;
use actix_web::web;

pub async fn handler(
    form: web::Form<AuthData>,
    state: web::Data<AppState>,
) -> actix_web::Result<web::Json<AuthenticableUser>, CustomError> {
    println!("form: {:?}", form);

    let form = AuthData {
        username: form.username.clone(),
        password: form.password.clone(),
    };

    let connection = state.db_pool.get()?;

    let api_slack_users = users::list_users::handler(state).await?;

    User::sync_api_users(&connection, api_slack_users.members.clone())?;

    let user = NewAuthenticableUser::create(&connection, form)?;

    Ok(web::Json(user))
}
