# Events Endpoint

**Endpoint:** `GET /events/`  
**Base URL:** `https://fantasy.premierleague.com/api/events/`

## Overview
This endpoint was not in the original documentation but works!

Returns an array of all gameweeks (events) with detailed statistics and metadata. This is extremely useful for getting gameweek-level data.

## Parameters
**No working parameters discovered** - Returns all 38 gameweeks regardless of query parameters.

## Response Format

```json
[
  {
    "id": 1,
    "name": "Gameweek 1", 
    "deadline_time": "2024-08-16T17:30:00Z",
    "release_time": null,
    "average_entry_score": 65,
    "finished": true,
    "data_checked": true,
    "highest_scoring_entry": 1234567,
    "deadline_time_epoch": 1723824600,
    "deadline_time_game_offset": 0,
    "highest_score": 142,
    "is_previous": true,
    "is_current": false,
    "is_next": false,
    "can_enter": false,
    "can_manage": false,
    "cup_leagues_created": true,
    "h2h_ko_matches_created": false,
    "ranked_count": 10567284,
    "chip_plays": [
      {
        "chip_name": "3xc",
        "num_played": 892456
      },
      {
        "chip_name": "wildcard", 
        "num_played": 234567
      }
    ],
    "most_selected": 302,
    "most_transferred_in": 145,
    "top_element": 302,
    "most_captained": 302,
    "most_vice_captained": 567,
    "transfers_made": 12567890,
    "top_element_info": {
      "id": 302,
      "points": 24
    }
  }
  // ... 37 more gameweeks
]
```

## Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | Integer | Gameweek number (1-38) |
| `name` | String | Display name ("Gameweek X") |
| `deadline_time` | String | ISO datetime when transfers lock |
| `deadline_time_epoch` | Integer | Unix timestamp of deadline |
| `average_entry_score` | Integer | Average points scored by all managers |
| `finished` | Boolean | Whether gameweek is completed |
| `data_checked` | Boolean | Whether data has been verified |
| `highest_scoring_entry` | Integer | Manager ID with highest score |
| `highest_score` | Integer | Highest individual score in gameweek |
| `is_previous` | Boolean | Whether this is the previous gameweek |
| `is_current` | Boolean | Whether this is the current gameweek |
| `is_next` | Boolean | Whether this is the next gameweek |
| `can_enter` | Boolean | Whether new managers can join |
| `can_manage` | Boolean | Whether teams can be managed |
| `cup_leagues_created` | Boolean | Cup competition status |
| `ranked_count` | Integer | Number of ranked managers |
| `chip_plays` | Array | Chips used in this gameweek |
| `most_selected` | Integer | Most selected player ID |
| `most_transferred_in` | Integer | Most transferred in player ID |
| `most_captained` | Integer | Most captained player ID |
| `most_vice_captained` | Integer | Most vice-captained player ID |
| `transfers_made` | Integer | Total transfers made |
| `top_element` | Integer | Highest scoring player ID |
| `top_element_info` | Object | Top player details and points |

## Usage Examples

### Get all gameweek data
```bash
curl "https://fantasy.premierleague.com/api/events/"
```

### Find current gameweek (in code)
```javascript
// Filter for is_current: true
const currentGameweek = events.find(event => event.is_current);
```

### Get gameweek statistics
```javascript
// Average score across all gameweeks
const avgScores = events.map(event => ({
  gameweek: event.id,
  avgScore: event.average_entry_score,
  highestScore: event.highest_score
}));
```

## Key Use Cases

1. **Current Gameweek Detection**: Use `is_current: true` to find active gameweek
2. **Deadline Tracking**: Monitor `deadline_time` for transfer deadlines  
3. **Statistics Analysis**: Track average scores, top performers
4. **Chip Usage Analytics**: See which chips are being played when
5. **Player Popularity**: Track most selected, captained, transferred players

## Integration Notes

1. **Gameweek Status**: Use boolean flags (`is_current`, `is_next`, `is_previous`) for navigation
2. **Player References**: Use player IDs with `/element-summary/{id}/` for player details
3. **Manager References**: Use `highest_scoring_entry` with `/entry/{id}/` for manager details
4. **Chip Strategy**: Analyze `chip_plays` array for chip usage patterns

## Related Endpoints
- `/bootstrap-static/` - Get player names for IDs referenced here
- `/element-summary/{id}/` - Get details for top players
- `/entry/{id}/` - Get details for top managers  
- `/event/{id}/live/` - Get live data for specific gameweek

## Caching Strategy
- **Update Frequency**: Once per gameweek or when deadline passes
- **Cache Duration**: 1-6 hours depending on gameweek status
- **Invalidation**: When `data_checked` changes from false to true
