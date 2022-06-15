use crate::error::CustomError;
use crate::models::slack_responses::SlackUser;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub slack_id: String,
    pub slack_uname: String,
}

impl User {
    /// function for fetching slack users from the database by their slack username
    pub fn find_by_username(
        connection: &PgConnection,
        slack_username: &str,
    ) -> Result<User, CustomError> {
        use crate::schema::users::dsl::*;

        let res = users
            .filter(slack_uname.eq(slack_username))
            .limit(1)
            .get_result(connection)?;

        Ok(res)
    }

    /// function for fetching all slack users stored in the database
    pub fn all(connection: &PgConnection) -> Result<Vec<User>, CustomError> {
        use crate::schema::users::dsl::*;

        let slack_users = users.load::<Self>(connection)?;
        Ok(slack_users)
    }

    pub fn sync_api_users(
        connection: &PgConnection,
        slack_users: Vec<SlackUser>,
    ) -> Result<Vec<Self>, CustomError> {
        let users = Self::all(connection)?;

        let mut users_to_insert: Vec<NewUser> = Vec::new();

        for slack_user in slack_users {
            let mut flag = false;
            for user in &users {
                if slack_user.id.eq(&user.slack_id) {
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

        NewUser::create_many(connection, users_to_insert)
    }
}

use crate::schema::users;
#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser {
    pub slack_id: String,
    pub slack_uname: String,
}

impl NewUser {
    /// function for inserting multiple slack users in the database
    pub fn create_many(
        conn: &PgConnection,
        slack_users: Vec<NewUser>,
    ) -> Result<Vec<User>, CustomError> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(slack_users)
            .get_results(conn)
            .map_err(CustomError::from)
    }
}
