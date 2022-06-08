use serde::{Deserialize, Serialize};

// chat.postMessage
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSent {
    pub channel: String,
    pub message: SlackMessage,
    pub ok: bool,
}

// message field in MessageSent
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessage {
    pub app_id: String,
    pub bot_id: String,
    pub subtype: String,
    pub text: String,
    pub r#type: String,
    pub username: String,
}

// conversations.list
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelsList {
    pub channels: Vec<SlackChannel>,
    pub ok: bool,
}

// conversations.info?channel={channel_id}
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleChannel {
    pub channel: SlackChannel,
    pub ok: bool,
}

// channel objects in the 'channels' field used in conversations.list and conversations.info?channel={channel_id}
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackChannel {
    pub created: usize,
    pub creator: String,
    pub id: String,
    pub name: String,
    pub name_normalized: String,
    pub parent_conversation: Option<String>,
    pub previous_names: Vec<String>,
    pub purpose: TopicPurpose,
    pub topic: TopicPurpose,
}
// topic and purpose fields in SlackChannel
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicPurpose {
    pub creator: String,
    pub last_set: usize,
    pub value: String,
}

// users.list
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersList {
    members: Vec<SlackUser>,
}

// user object in the 'members' field of users.list
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackUser {
    id: String,
    is_admin: bool,
    name: String,
    real_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackError {
    ok: bool,
    error: String,
}
