use crate::state::app::AppState;
use actix_web::web;
use crate::routes::{users};
use barrage_slacker::{CustomError, register_user, fetch_slack_users, insert_slack_users, AuthData};
use barrage_slacker::models::authenticable_users::AuthenticableUser;
use bcrypt::{DEFAULT_COST, hash};
use barrage_slacker::models::user::{NewUser};

pub async fn handler (
    form: web::Form<AuthData>,
    state: web::Data<AppState>
) -> actix_web::Result<web::Json<AuthenticableUser>, CustomError> {
    println!("form: {:?}", form);

    let hashed_password = hash(form.password.clone(), DEFAULT_COST).expect("Unable to hash the password");

    let form = AuthData {
        username: form.username.clone(),
        password: hashed_password,
    };

    let connection = state.db_pool.get().expect("Couldn't get pool conn");
    let db_slack_users = fetch_slack_users(&connection)?;
    let api_slack_users = users::list_users::handler(state).await?;
    let mut users_to_insert: Vec<NewUser> = Vec::new();

    for slack_user in api_slack_users.members.clone() {
        let mut flag = false;
        for db_user in &db_slack_users {
            if slack_user.id.eq(&db_user.slack_id) {
                flag = true;
            }
        }

        if !flag {
            let new_user = NewUser {
                slack_id: slack_user.id,
                slack_uname: slack_user.name,
            };
            users_to_insert.push(new_user);
        }
    }

    insert_slack_users(&connection, users_to_insert)?;
    let user = register_user(&connection, &form.username, &form.password)?;
    Ok(web::Json(user))
}