table! {
    channels (id) {
        id -> Varchar,
        channel_name -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Varchar,
        sender -> Varchar,
        body -> Varchar,
        channel -> Varchar,
        time_sent -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        slack_id -> Varchar,
        slack_uname -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    users,
);
