use crate::schema::channels;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use crate::error::CustomError;
use crate::models::slack_responses::SlackChannel;

#[derive(Queryable)]
pub struct Channel {
    pub id: String,
    pub channel_name: String,
}

impl Channel {
    pub fn find_by_channel_id(connection: &PgConnection, id: String) -> Result<Channel, CustomError> {
        let channel = channels::table.find(id).first(connection)?;
        Ok(channel)
    }

    pub fn all(connection: &PgConnection) -> Result<Vec<Channel>, CustomError> {
        use crate::schema::channels::dsl::*;

        let db_channels = channels.load::<Self>(connection)?;
        Ok(db_channels)
    }

    pub fn sync_api_channels(connection: &PgConnection, slack_channels:Vec<SlackChannel>) -> Result<Vec<Self>, CustomError> {
        let db_channels = Self::all(connection)?;

        let mut channels_to_insert: Vec<NewChannel> = Vec::new();

        for slack_channel in slack_channels {
            let mut flag = false;
            for db_channel in &db_channels {
                if db_channel.id.eq(&slack_channel.id) {
                    flag = true;
                }
            }

            if !flag {
                let new_channel = NewChannel {
                    channel_id: slack_channel.id,
                    channel_name: slack_channel.name
                };
                channels_to_insert.push(new_channel);
            }
        }

        NewChannel::create_many(connection, channels_to_insert)
    }
}


#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewChannel {
    pub channel_id: String,
    pub channel_name: String,
}

impl NewChannel {
    pub fn create_many(connection: &PgConnection, slack_channels: Vec<NewChannel>) -> Result<Vec<Channel>, CustomError> {
        use crate::schema::channels::dsl::*;

        diesel::insert_into(channels)
        .values(slack_channels)
        .get_results(connection)
        .map_err(CustomError::from)
    }
}
