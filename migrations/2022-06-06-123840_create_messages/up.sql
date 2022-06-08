CREATE TABLE messages (
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    sender VARCHAR(36) NOT NULL,
    body VARCHAR(500) NOT NULL,
    channel VARCHAR(11) NOT NULL,
    time_sent TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);