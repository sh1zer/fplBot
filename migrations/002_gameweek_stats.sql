create table gameweek_player_stats (
  gameweek   integer not null,
  player_id  integer not null,

  minutes integer not null,
  goals_scored integer not null,
  assists integer not null,
  clean_sheets integer not null,
  goals_conceded integer not null,
  own_goals integer not null,
  penalties_saved integer not null,
  penalties_missed integer not null,
  yellow_cards integer not null,
  red_cards integer not null,
  saves integer not null,
  bonus integer not null,
  bps integer not null,
  clearances_blocks_interceptions integer not null,
  recoveries integer not null,
  tackles integer not null,
  defensive_contribution integer not null,
  starts integer not null,

  influence real not null,
  creativity real not null,
  threat real not null,
  ict_index real not null,
  expected_goals real not null,
  expected_assists real not null,
  expected_goal_involvements real not null,
  expected_goals_conceded real not null,

  total_points integer not null,
  in_dreamteam integer not null,   -- bool 0/1
  modified integer not null,       -- bool 0/1
  primary key (gameweek, player_id)
);

create index idx_gw_points on gameweek_player_stats (gameweek, total_points desc);
create index idx_player_gw on gameweek_player_stats (player_id, gameweek);

create table gameweek_player_points_breakdown (
  gameweek            integer not null,
  player_id           integer not null,
  fixture_id          integer not null,
  identifier          text    not null,
  points              integer not null,
  value               integer not null,
  points_modification integer not null,
  primary key (gameweek, player_id, fixture_id, identifier),
  foreign key (gameweek, player_id)
      references gameweek_player_stats (gameweek, player_id)
      on delete cascade
);

create index idx_breakdown_player_fixture
  on gameweek_player_points_breakdown (gameweek, player_id, fixture_id);
create index idx_breakdown_fixture
  on gameweek_player_points_breakdown (gameweek, fixture_id);
