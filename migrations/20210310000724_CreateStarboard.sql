-- Add migration script here
-- TODO: banned users, whitelist/blacklist specific channels
CREATE TABLE IF NOT EXISTS starboard (
  -- the ID of the starboard
  id SERIAL PRIMARY KEY,
  -- the channel id of the starboard
  channel_id BIGINT NOT NULL,
  -- the guild id the starboard is in
  guild_id BIGINT NOT NULL,
  -- the emoji to react with to add a message
  emoji VARCHAR(64) NOT NULL,
  -- when the starboard was created
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
