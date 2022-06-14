#[derive(Queryable)]
pub struct Channel {
    pub id: String,
    pub channel_name: String,
}
use crate::schema::channels;

#[derive(Insertable)]
#[table_name = "channels"]
pub struct NewChannel<'a> {
    pub channel_name: &'a str,
}
