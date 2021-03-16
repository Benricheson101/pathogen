-- Add migration script here
--  CREATE TABLE IF NOT EXISTS configs (
--    id BIGINT NOT NULL PRIMARY KEY,
--    prefix VARCHAR(32)
--  );
CREATE TABLE configs (
  -- config ID
  id SERIAL PRIMARY KEY,
  -- the id of the guild the config blongs to
  guild_id BIGINT NOT NULL,
  -- if the config is active or not
  is_active BOOLEAN NOT NULL,
  -- when the config was added
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  -- who edited the config
  created_by BIGINT NOT NULL,
  -- the config (json)
  config jsonb NOT NULL
);

CREATE UNIQUE INDEX ON configs (guild_id)
WHERE is_active;
