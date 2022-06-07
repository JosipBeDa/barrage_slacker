CREATE TABLE messages (
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    sender VARCHAR(36),
    body VARCHAR(500),
    channel VARCHAR(11),
    time_sent TIMESTAMPTZ,
    PRIMARY KEY (id),
    FOREIGN KEY (sender) REFERENCES users(id),
    FOREIGN KEY (channel) REFERENCES channels(id)
);