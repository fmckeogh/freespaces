-- Add migration script here
CREATE TABLE IF NOT EXISTS images (
    filename VARCHAR(255) PRIMARY KEY,
    contents BYTEA NOT NULL
);
