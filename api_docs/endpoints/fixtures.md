# Fixtures Endpoint

**Endpoint:** `GET /fixtures/`
**Base URL:** `https://fantasy.premierleague.com/api/fixtures/`

## Overview
Returns fixture data for the Premier League season. This is one of the most useful endpoints as it supports actual filtering parameters.

## Parameters

### Working Parameters

#### `event` (gameweek filter)
- **Type:** Integer (1-38)
- **Description:** Filter fixtures by gameweek number
- **Returns:** ~10 fixtures per gameweek
- **Example:** `/fixtures/?event=1`

#### `team` (team filter)  
- **Type:** Integer (1-20)
- **Description:** Filter to show all fixtures for a specific team
- **Returns:** 38 fixtures (19 home + 19 away)
- **Example:** `/fixtures/?team=1`

### Non-Working Parameters
These parameters are accepted but ignored:
- `finished` - Returns all fixtures regardless of completion status
- `team_h` - Ignored (use `team` instead)
- `team_a` - Ignored (use `team` instead)  
- `limit` - Ignored (returns all matching fixtures)
- `page` - Ignored
- `format` - Only JSON supported

## Response Format

### Base Response (no parameters)
```json
[
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
    "stats": [
      {
        "identifier": "goals_scored",
        "a": [
          {
            "value": 1,
            "element": 123
          }
        ],
        "h": [
          {
            "value": 2,
            "element": 456
          }
        ]
      }
    ],
    "team_h_difficulty": 3,
    "team_a_difficulty": 2,
    "pulse_id": 123456
  }
  // ... 379 more fixtures
]
```

### With `?event=1` (10 items)
Same structure as above, but filtered to gameweek 1 fixtures only.

### With `?team=1` (38 items)  
Same structure, but only fixtures where `team_h: 1` OR `team_a: 1`.

## Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | Integer | Unique fixture ID |
| `code` | Integer | FPL fixture code |
| `event` | Integer | Gameweek number (1-38) |
| `team_h` | Integer | Home team ID |
| `team_a` | Integer | Away team ID |
| `team_h_score` | Integer/null | Home team score (null if not started) |
| `team_a_score` | Integer/null | Away team score (null if not started) |
| `finished` | Boolean | Whether fixture is completed |
| `finished_provisional` | Boolean | Whether final scores are confirmed |
| `started` | Boolean | Whether fixture has kicked off |
| `kickoff_time` | String | ISO datetime of kickoff |
| `minutes` | Integer | Minutes played |
| `team_h_difficulty` | Integer | FPL difficulty rating (1-5) for home team |
| `team_a_difficulty` | Integer | FPL difficulty rating (1-5) for away team |
| `stats` | Array | Match statistics (goals, assists, cards, etc.) |
| `provisional_start_time` | Boolean | Whether kickoff time is provisional |
| `pulse_id` | Integer | External match ID |

## Usage Examples

### Get all fixtures for current gameweek
```bash
curl "https://fantasy.premierleague.com/api/fixtures/?event=4"
```

### Get all Arsenal fixtures (assuming team_id=1)
```bash  
curl "https://fantasy.premierleague.com/api/fixtures/?team=1"
```

### Get all fixtures (no filter)
```bash
curl "https://fantasy.premierleague.com/api/fixtures/"
```

## Implementation Notes

1. **Team ID Mapping**: Get team IDs from `/bootstrap-static/` endpoint's `teams` array
2. **Current Gameweek**: Get current gameweek from `/bootstrap-static/` endpoint's `events` array (look for `is_current: true`)
3. **Caching**: Fixture data changes infrequently - cache responses appropriately
4. **Filtering**: This is the ONLY endpoint that supports meaningful filtering parameters

## Rate Limiting
No specific rate limits observed, but implement reasonable delays between requests.

## Related Endpoints
- `/bootstrap-static/` - Get team names and current gameweek
- `/event/{id}/live/` - Get live scores and player performance for gameweek
