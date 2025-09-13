# Common Response Schemas

This document contains the detailed schemas for common response objects used across FPL API endpoints.

## Bootstrap Static Response

The main configuration endpoint that contains all static data.

```json
{
  "events": [Event],
  "game_settings": GameSettings,
  "phases": [Phase],
  "teams": [Team],
  "total_players": Integer,
  "elements": [Player],
  "element_stats": [ElementStat],
  "element_types": [ElementType],
  "chips": [Chip]
}
```

### Event Schema
```json
{
  "id": Integer,
  "name": String,
  "deadline_time": String (ISO datetime),
  "release_time": String | null,
  "average_entry_score": Integer,
  "finished": Boolean,
  "data_checked": Boolean,
  "highest_scoring_entry": Integer,
  "deadline_time_epoch": Integer,
  "deadline_time_game_offset": Integer,
  "highest_score": Integer,
  "is_previous": Boolean,
  "is_current": Boolean,
  "is_next": Boolean,
  "can_enter": Boolean,
  "can_manage": Boolean,
  "cup_leagues_created": Boolean,
  "h2h_ko_matches_created": Boolean,
  "ranked_count": Integer,
  "chip_plays": [ChipPlay],
  "most_selected": Integer (player_id),
  "most_transferred_in": Integer (player_id),
  "top_element": Integer (player_id),
  "most_captained": Integer (player_id),
  "most_vice_captained": Integer (player_id),
  "transfers_made": Integer,
  "top_element_info": {
    "id": Integer,
    "points": Integer
  }
}
```

### Team Schema
```json
{
  "code": Integer,
  "draw": Integer,
  "form": null,
  "id": Integer,
  "loss": Integer,
  "name": String,
  "played": Integer,
  "points": Integer,
  "position": Integer,
  "short_name": String,
  "strength": Integer,
  "team_division": null,
  "unavailable": Boolean,
  "win": Integer,
  "strength_overall_home": Integer,
  "strength_overall_away": Integer,
  "strength_attack_home": Integer,
  "strength_attack_away": Integer,
  "strength_defence_home": Integer,
  "strength_defence_away": Integer,
  "pulse_id": Integer
}
```

### Player Schema
```json
{
  "chance_of_playing_next_round": Integer | null,
  "chance_of_playing_this_round": Integer | null,
  "code": Integer,
  "cost_change_event": Integer,
  "cost_change_event_fall": Integer,
  "cost_change_start": Integer,
  "cost_change_start_fall": Integer,
  "dreamteam_count": Integer,
  "element_type": Integer,
  "ep_next": String,
  "ep_this": String,
  "event_points": Integer,
  "first_name": String,
  "form": String,
  "id": Integer,
  "in_dreamteam": Boolean,
  "news": String,
  "news_added": String | null,
  "now_cost": Integer,
  "photo": String,
  "points_per_game": String,
  "second_name": String,
  "selected_by_percent": String,
  "special": Boolean,
  "squad_number": Integer | null,
  "status": String,
  "team": Integer,
  "team_code": Integer,
  "total_points": Integer,
  "transfers_in": Integer,
  "transfers_in_event": Integer,
  "transfers_out": Integer,
  "transfers_out_event": Integer,
  "value_form": String,
  "value_season": String,
  "web_name": String,
  "minutes": Integer,
  "goals_scored": Integer,
  "assists": Integer,
  "clean_sheets": Integer,
  "goals_conceded": Integer,
  "own_goals": Integer,
  "penalties_saved": Integer,
  "penalties_missed": Integer,
  "yellow_cards": Integer,
  "red_cards": Integer,
  "saves": Integer,
  "bonus": Integer,
  "bps": Integer,
  "influence": String,
  "creativity": String,
  "threat": String,
  "ict_index": String,
  "starts": Integer,
  "expected_goals": String,
  "expected_assists": String,
  "expected_goal_involvements": String,
  "expected_goals_conceded": String,
  "influence_rank": Integer,
  "influence_rank_type": Integer,
  "creativity_rank": Integer,
  "creativity_rank_type": Integer,
  "threat_rank": Integer,
  "threat_rank_type": Integer,
  "ict_index_rank": Integer,
  "ict_index_rank_type": Integer,
  "corners_and_indirect_freekicks_order": Integer | null,
  "corners_and_indirect_freekicks_text": String,
  "direct_freekicks_order": Integer | null,
  "direct_freekicks_text": String,
  "penalties_order": Integer | null,
  "penalties_text": String
}
```

## Fixture Schema

```json
{
  "code": Integer,
  "event": Integer,
  "finished": Boolean,
  "finished_provisional": Boolean,
  "id": Integer,
  "kickoff_time": String (ISO datetime),
  "minutes": Integer,
  "provisional_start_time": Boolean,
  "started": Boolean,
  "team_a": Integer,
  "team_a_score": Integer | null,
  "team_h": Integer,
  "team_h_score": Integer | null,
  "stats": [FixtureStat],
  "team_h_difficulty": Integer,
  "team_a_difficulty": Integer,
  "pulse_id": Integer
}
```

### FixtureStat Schema
```json
{
  "identifier": String,
  "a": [StatEntry],
  "h": [StatEntry]
}
```

### StatEntry Schema
```json
{
  "value": Integer,
  "element": Integer (player_id)
}
```

## Manager/Entry Schema

```json
{
  "id": Integer,
  "joined_time": String (ISO datetime),
  "started_event": Integer,
  "favourite_team": Integer,
  "player_first_name": String,
  "player_last_name": String,
  "player_region_id": Integer,
  "player_region_name": String,
  "player_region_iso_code_short": String,
  "player_region_iso_code_long": String,
  "summary_overall_points": Integer,
  "summary_overall_rank": Integer,
  "summary_event_points": Integer,
  "summary_event_rank": Integer,
  "current_event": Integer,
  "leagues": {
    "classic": [LeagueEntry],
    "h2h": [LeagueEntry],
    "cup": CupStatus,
    "cup_matches": [CupMatch]
  },
  "name": String,
  "name_change_blocked": Boolean,
  "kit": Kit,
  "last_deadline_bank": Integer,
  "last_deadline_value": Integer,
  "last_deadline_total_transfers": Integer
}
```

## League Standing Schema

```json
{
  "new_entries": {
    "has_next": Boolean,
    "page": Integer,
    "results": [LeagueEntry]
  },
  "last_updated_data": String (ISO datetime),
  "league": {
    "id": Integer,
    "name": String,
    "created": String (ISO datetime),
    "closed": Boolean,
    "max_entries": Integer | null,
    "league_type": String,
    "scoring": String,
    "admin_entry": Integer,
    "start_event": Integer,
    "code_privacy": String,
    "has_cup": Boolean,
    "cup_league": Integer | null,
    "rank": Integer | null
  },
  "standings": {
    "has_next": Boolean,
    "page": Integer,
    "results": [StandingEntry]
  }
}
```

### StandingEntry Schema
```json
{
  "id": Integer,
  "event_total": Integer,
  "player_name": String,
  "rank": Integer,
  "last_rank": Integer,
  "rank_sort": Integer,
  "total": Integer,
  "entry": Integer,
  "entry_name": String
}
```

## Live Event Schema

```json
{
  "elements": [LivePlayerStat]
}
```

### LivePlayerStat Schema
```json
{
  "id": Integer,
  "stats": {
    "minutes": Integer,
    "goals_scored": Integer,
    "assists": Integer,
    "clean_sheets": Integer,
    "goals_conceded": Integer,
    "own_goals": Integer,
    "penalties_saved": Integer,
    "penalties_missed": Integer,
    "yellow_cards": Integer,
    "red_cards": Integer,
    "saves": Integer,
    "bonus": Integer,
    "bps": Integer,
    "influence": String,
    "creativity": String,
    "threat": String,
    "ict_index": String,
    "starts": Integer,
    "expected_goals": String,
    "expected_assists": String,
    "expected_goal_involvements": String,
    "expected_goals_conceded": String,
    "total_points": Integer,
    "in_dreamteam": Boolean
  },
  "explain": [PointsExplanation]
}
```

### PointsExplanation Schema
```json
{
  "fixture": Integer,
  "stats": [
    {
      "identifier": String,
      "points": Integer,
      "value": Integer
    }
  ]
}
```

## Transfer Schema

```json
{
  "element_in": Integer (player_id),
  "element_in_cost": Integer,
  "element_out": Integer (player_id),
  "element_out_cost": Integer,
  "entry": Integer (manager_id),
  "event": Integer (gameweek),
  "time": String (ISO datetime)
}
```

## Manager History Schema

```json
{
  "current": [GameweekHistory],
  "past": [SeasonHistory],
  "chips": [ChipUsage]
}
```

### GameweekHistory Schema
```json
{
  "event": Integer,
  "points": Integer,
  "total_points": Integer,
  "rank": Integer,
  "rank_sort": Integer,
  "overall_rank": Integer,
  "bank": Integer,
  "value": Integer,
  "event_transfers": Integer,
  "event_transfers_cost": Integer,
  "points_on_bench": Integer
}
```

### SeasonHistory Schema
```json
{
  "season_name": String,
  "total_points": Integer,
  "rank": Integer
}
```

### ChipUsage Schema
```json
{
  "name": String,
  "time": String (ISO datetime),
  "event": Integer
}
```

## Common Field Types

### Status Codes
- `"a"` - Available
- `"d"` - Doubtful  
- `"i"` - Injured
- `"s"` - Suspended
- `"u"` - Unavailable

### League Types
- `"s"` - Standard/Classic (total points)
- `"x"` - Head-to-head

### Position Types
- `1` - Goalkeeper
- `2` - Defender
- `3` - Midfielder
- `4` - Forward