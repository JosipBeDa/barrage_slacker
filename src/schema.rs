table! {
    messages (id) {
        id -> Int4,
        sender -> Varchar,
        body -> Text,
        time_sent -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        slack_id -> Varchar,
        slack_uname -> Varchar,
    }
}

table! {
    channels (id) {
        id -> Varchar,
        channel_name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
    channels
);
