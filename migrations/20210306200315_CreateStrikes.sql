-- Add migration script here
CREATE TYPE strike_type AS ENUM ('ban', 'kick', 'warn', 'mute');

CREATE TABLE IF NOT EXISTS strikes (
  -- strike ID
  id SERIAL PRIMARY KEY,
  -- guild id
  guild_id BIGINT NOT NULL,
  -- user who received the strike
  target BIGINT NOT NULL,
  -- user who created the strike
  moderator BIGINT NOT NULL,
  -- the type of strike
  kind strike_type NOT NULL,
  -- reason given (default is handled in the bot's code)
  reason TEXT,
  -- when the strike was created
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  -- the time of the most recent edit
  edited_at TIMESTAMPTZ
);
