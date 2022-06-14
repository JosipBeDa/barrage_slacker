CREATE TABLE messages (
    message_id SERIAL,
    sender VARCHAR(36) NOT NULL,
    body VARCHAR(500) NOT NULL,
    channel VARCHAR(11) NOT NULL,
    time_sent TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (message_id)
);