table! {
    authenticable_users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    channels (channel_id) {
        channel_id -> Varchar,
        channel_name -> Varchar,
    }
}

table! {
    messages (message_id) {
        message_id -> Int4,
        sender -> Varchar,
        body -> Varchar,
        channel -> Varchar,
        time_sent -> Timestamptz,
    }
}

table! {
    users (slack_id) {
        slack_id -> Varchar,
        slack_uname -> Varchar,
    }
}

joinable!(messages -> channels (channel));
joinable!(messages -> users (sender));

allow_tables_to_appear_in_same_query!(
    authenticable_users,
    channels,
    messages,
    users,
);
