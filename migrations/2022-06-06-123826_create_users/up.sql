CREATE TABLE users (
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    slack_id VARCHAR(255) NOT NULL,
    slack_uname VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);