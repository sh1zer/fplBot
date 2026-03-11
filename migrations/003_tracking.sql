CREATE TABLE channel_tracked_fixtures (
  channel_id INTEGER NOT NULL,
  fixture_id INTEGER NOT NULL,
  PRIMARY KEY (channel_id, fixture_id)
);

CREATE TABLE channel_tracked_players (
  channel_id INTEGER NOT NULL,
  player_id INTEGER NOT NULL,
  PRIMARY KEY (channel_id, player_id)
);

CREATE INDEX idx_tracking_fixture ON channel_tracked_fixtures (fixture_id);

CREATE INDEX idx_tracking_player ON channel_tracked_players (player_id);
