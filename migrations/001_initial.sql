CREATE TABLE users(
  discord_id INTEGER PRIMARY KEY,
  manager_id INTEGER
);

CREATE TABLE channels(
  channel_id INTEGER PRIMARY KEY,
  default_league_id INTEGER
);
