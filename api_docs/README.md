# FPL API Comprehensive Documentation

**Base URL:** `https://fantasy.premierleague.com/api`

## Table of Contents
- [Key Discoveries](#key-discoveries)
- [Endpoint Reference](#endpoint-reference)
- [Working Parameters](#working-parameters)
- [Response Schemas](#response-schemas)
- [Authentication](#authentication)

## Key Discoveries

### 🎯 Major Findings:
1. **The `fixtures/` endpoint supports powerful filtering:**
   - `?event=X` - Returns fixtures for specific gameweek (10 per GW)
   - `?team=X` - Returns all fixtures for a specific team (38 total)

2. **The `events/` endpoint exists and works!** (Not in original docs)
   - Returns array of all 38 gameweeks with detailed statistics

3. **Most query parameters are ignored** - API returns full datasets regardless
   - `?limit`, `?page`, `?fields`, etc. are accepted but don't filter results
   - Exception: `fixtures/` endpoint respects `event` and `team` parameters

4. **All endpoints return JSON only** - `?format=xml` returns 404

5. **No authentication required** for most endpoints (including some documented as "auth required")

## Endpoint Reference

### Core Data Endpoints

#### `GET /bootstrap-static/`
**Master dataset with all FPL configuration data**

- **Description:** Complete FPL static data including teams, players, gameweeks, and settings
- **Response Size:** Large (~2MB JSON object)
- **Key Data:**
  - 38 gameweeks/events
  - 20 teams 
  - 738 players
  - Game settings and chip configurations

**Response Structure:**
```json
{
  "events": [...],        // 38 gameweeks
  "teams": [...],         // 20 Premier League teams  
  "elements": [...],      // 738 players
  "element_types": [...], // Position types (GK, DEF, MID, FWD)
  "element_stats": [...], // Player stat definitions
  "game_settings": {...}, // FPL configuration
  "phases": [...],        // Season phases
  "chips": [...],         // Available chips
  "total_players": 10567284
}
```

#### `GET /fixtures/`
**All fixtures for the season**

- **Response:** Array of 380 fixtures
- **Supports Filtering:** ✅ (see parameters below)

**Working Parameters:**
- `?event={gameweek_id}` - Filter by gameweek (returns ~10 fixtures)
- `?team={team_id}` - Filter by team (returns 38 fixtures for that team)

**Examples:**
```bash
# All fixtures (380 items)
/fixtures/

# Gameweek 1 fixtures only (10 items)
/fixtures/?event=1

# All Arsenal fixtures (team_id=1) (38 items)  
/fixtures/?team=1
```

#### `GET /events/`
**All gameweeks with statistics** 🆕

- **Description:** Array of all 38 gameweeks with detailed stats
- **Response:** Array of 38 gameweek objects
- **Key Data Per Gameweek:**
  - Average entry score
  - Deadline time
  - Chip play counts
  - Most selected/captained players
  - Transfer statistics

#### `GET /event/{gameweek_id}/live/`
**Live gameweek data for all players**

- **Description:** Real-time stats for all players in a specific gameweek
- **Response Structure:**
```json
{
  "elements": [
    {
      "id": 1,
      "stats": {
        "minutes": 90,
        "goals_scored": 1,
        "assists": 0,
        "clean_sheets": 0,
        "goals_conceded": 1,
        "bonus": 2,
        "bps": 32,
        "total_points": 7
      }
    }
    // ... for all 738 players
  ]
}
```

### Player & Manager Endpoints

#### `GET /element-summary/{player_id}/`
**Individual player detailed data**

- **Response Structure:**
```json
{
  "fixtures": [...],      // Upcoming fixtures
  "history": [...],       // Current season gameweek history
  "history_past": [...]   // Previous seasons summary
}
```

#### `GET /entry/{manager_id}/`
**Manager profile and summary**

- **Description:** Public manager information
- **Response:** Object with 25+ fields including:
  - Manager name and team details
  - Current gameweek info
  - Favourite team
  - Total points and rank

#### `GET /entry/{manager_id}/history/`
**Manager's historical performance**

- **Response Structure:**
```json
{
  "current": [...],    // Current season gameweek-by-gameweek
  "past": [...],       // Previous seasons summary  
  "chips": [...]       // Chips used this season
}
```

#### `GET /entry/{manager_id}/picks/`
**Manager's current team**

- **Alternative endpoint:** `/my-team/{manager_id}/`
- **Description:** Current squad selection and formation

#### `GET /entry/{manager_id}/event/{gameweek_id}/picks/`
**Manager's team for specific gameweek**

- **Description:** Historical team selection for any completed gameweek

#### `GET /entry/{manager_id}/transfers/`
**All manager transfers this season**

- **Response:** Array of all transfers made

### League Endpoints

#### `GET /leagues-classic/{league_id}/standings/`
**Classic league standings**

**Working Parameters:**
- `?page_standings={page}` - Paginate main standings
- `?page_new_entries={page}` - Paginate new entries
- `?phase={phase_id}` - Filter by phase/mini-league period

**Response Structure:**
```json
{
  "standings": {
    "page": 1,
    "has_next": true,
    "results": [...]
  },
  "league": {...},           // League metadata
  "new_entries": {...},      // New entries pagination
  "last_updated_data": "..." // Timestamp
}
```

### Status & Metadata

#### `GET /event-status/`
**Current gameweek status**

- **Response:**
```json
{
  "status": [...],    // Bonus points status
  "leagues": "Updated" // League update status
}
```

## Working Parameters

### Confirmed Working Parameters:

#### `fixtures/` endpoint:
- ✅ `?event={1-38}` - Filter by gameweek
- ✅ `?team={1-20}` - Filter by team ID
- ❌ `?finished=true/false` - Ignored (returns all)
- ❌ `?team_h={id}` - Ignored  
- ❌ `?team_a={id}` - Ignored

#### `leagues-classic/{id}/standings/` endpoint:
- ✅ `?page_standings={page}` - Paginate standings
- ✅ `?page_new_entries={page}` - Paginate new entries  
- ✅ `?phase={phase_id}` - Filter by phase

#### Other endpoints:
- ❌ Most endpoints ignore query parameters
- ❌ `?limit`, `?offset`, `?page`, `?fields` are universally ignored
- ❌ `?format=xml` returns 404 (only JSON supported)

## Response Schemas

### Common Patterns:

**Fixture Object:**
```json
{
  "code": 2210000,
  "event": 1,
  "finished": true,
  "finished_provisional": true,
  "id": 1,
  "kickoff_time": "2024-08-16T19:00:00Z",
  "minutes": 90,
  "provisional_start_time": false,
  "started": true,
  "team_a": 7,
  "team_a_score": 1, 
  "team_h": 2,
  "team_h_score": 2,
  "stats": [...],     // Match statistics
  "team_h_difficulty": 3,
  "team_a_difficulty": 2
}
```

**Player Object (from bootstrap-static):**
```json
{
  "id": 1,
  "photo": "1.jpg",
  "web_name": "Player",
  "team_code": 3,
  "status": "a",
  "code": 12345,
  "first_name": "First",
  "second_name": "Last",
  "squad_number": null,
  "news": "",
  "now_cost": 85,
  "news_added": null,
  "chance_of_playing_this_round": 100,
  "chance_of_playing_next_round": 100,
  "value_form": "0.0",
  "value_season": "16.7",
  "cost_change_start": 0,
  "cost_change_event": 0,
  "cost_change_start_fall": 0,
  "cost_change_event_fall": 0,
  "in_dreamteam": false,
  "dreamteam_count": 0,
  "selected_by_percent": "20.3",
  "form": "0.0",
  "transfers_out": 0,
  "transfers_in": 0,
  "transfers_out_event": 0,
  "transfers_in_event": 0,
  "loans_in": 0,
  "loans_out": 0,
  "loaned_in": 0,
  "loaned_out": 0,
  "total_points": 0,
  "event_points": 0,
  "points_per_game": "0.0",
  "ep_this": "0.0",
  "ep_next": "0.0",
  "special": false,
  "minutes": 0,
  "goals_scored": 0,
  "assists": 0,
  "clean_sheets": 0,
  "goals_conceded": 0,
  "own_goals": 0,
  "penalties_saved": 0,
  "penalties_missed": 0,
  "yellow_cards": 0,
  "red_cards": 0,
  "saves": 0,
  "bonus": 0,
  "bps": 0,
  "influence": "0.0",
  "creativity": "0.0",
  "threat": "0.0",
  "ict_index": "0.0",
  "starts": 0,
  "expected_goals": "0.0",
  "expected_assists": "0.0",
  "expected_goal_involvements": "0.0",
  "expected_goals_conceded": "0.0",
  "influence_rank": 0,
  "influence_rank_type": 0,
  "creativity_rank": 0,
  "creativity_rank_type": 0,
  "threat_rank": 0,
  "threat_rank_type": 0,
  "ict_index_rank": 0,
  "ict_index_rank_type": 0,
  "corners_and_indirect_freekicks_order": null,
  "corners_and_indirect_freekicks_text": "",
  "direct_freekicks_order": null,
  "direct_freekicks_text": "",
  "penalties_order": null,
  "penalties_text": "",
  "element_type": 2,
  "team": 1
}
```

## Authentication

**Surprising Discovery:** Most endpoints that were documented as requiring authentication actually work without it:

- ✅ `my-team/{manager_id}/` - Works without auth
- ✅ `entry/{manager_id}/transfers-latest/` - Works without auth  
- ✅ `me/` - Works without auth

This suggests the API may have relaxed authentication requirements or these endpoints return public data when accessed without auth.

## Team IDs Reference

Based on the API responses, team IDs 1-20 correspond to Premier League teams. The exact mapping would need to be determined from the `/bootstrap-static/` endpoint's `teams` array.

## Rate Limiting

No specific rate limits were encountered during testing, but it's recommended to implement reasonable delays between requests to avoid potential throttling.

## Next Steps for Integration

1. **Prioritize `fixtures/` endpoint filtering** - This provides the most useful parameter functionality
2. **Use `events/` endpoint** for gameweek metadata (this was not in original docs!)
3. **Implement `bootstrap-static/` caching** - Large response that changes infrequently
4. **Focus on team-specific queries** using `?team={id}` parameter