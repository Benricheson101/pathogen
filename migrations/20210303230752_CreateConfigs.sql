-- Add migration script here
CREATE TABLE IF NOT EXISTS configs (
  id BIGINT NOT NULL PRIMARY KEY,
  prefix VARCHAR(32)
);
