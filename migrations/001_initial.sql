CREATE TABLE users(
  discord_id TEXT PRIMARY KEY,
  manager_id INTEGER
);

CREATE TABLE channels(
  channel_id TEXT PRIMARY KEY,
  default_league_id INTEGER
);
