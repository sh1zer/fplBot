# League Standings Endpoint

**Endpoint:** `GET /leagues-classic/{league_id}/standings/`
**Base URL:** `https://fantasy.premierleague.com/api/leagues-classic/{league_id}/standings/`

## Overview
Returns standings for classic (total points) leagues. This endpoint supports several useful parameters for pagination and filtering.

## Parameters

### Working Parameters

#### `page_standings` 
- **Type:** Integer
- **Description:** Paginate the main league standings
- **Default:** 1
- **Example:** `/leagues-classic/314/standings/?page_standings=2`

#### `page_new_entries`
- **Type:** Integer  
- **Description:** Paginate new entries section
- **Default:** 1
- **Example:** `/leagues-classic/314/standings/?page_new_entries=1`

#### `phase`
- **Type:** Integer
- **Description:** Filter by league phase/period
- **Values:** Varies by league type
- **Example:** `/leagues-classic/314/standings/?phase=1`

### Non-Working Parameters
- `limit` - Ignored
- `event` - Ignored
- `has_entry` - Ignored

## Response Format

```json
{
  "new_entries": {
    "has_next": false,
    "page": 1,
    "results": []
  },
  "last_updated_data": "2024-08-17T14:23:45Z",
  "league": {
    "id": 314,
    "name": "Overall",
    "created": "2024-07-09T10:00:00Z",
    "closed": false,
    "max_entries": null,
    "league_type": "s",
    "scoring": "c",
    "admin_entry": 1,
    "start_event": 1,
    "code_privacy": "p",
    "has_cup": false,
    "cup_league": null,
    "rank": null
  },
  "standings": {
    "has_next": true,
    "page": 1,
    "results": [
      {
        "id": 12345,
        "event_total": 87,
        "player_name": "John Smith",
        "rank": 1,
        "last_rank": 2,
        "rank_sort": 1,
        "total": 654,
        "entry": 1234567,
        "entry_name": "Team Name"
      }
      // ... more entries
    ]
  }
}
```

## Field Descriptions

### League Object
| Field | Type | Description |
|-------|------|-------------|
| `id` | Integer | League ID |
| `name` | String | League name |
| `created` | String | League creation date |
| `closed` | Boolean | Whether league is closed to new entries |
| `max_entries` | Integer/null | Maximum allowed entries |
| `league_type` | String | "s" for standard, "x" for head-to-head |
| `scoring` | String | Scoring type |
| `admin_entry` | Integer | League admin's entry ID |
| `start_event` | Integer | Starting gameweek |

### Standings Results
| Field | Type | Description |
|-------|------|-------------|
| `id` | Integer | Standing entry ID |
| `entry` | Integer | Manager's entry ID |
| `entry_name` | String | Team name |
| `player_name` | String | Manager's name |
| `rank` | Integer | Current rank in league |
| `last_rank` | Integer | Previous rank |
| `rank_sort` | Integer | Sorting rank |
| `total` | Integer | Total points |
| `event_total` | Integer | Points from last gameweek |

### Pagination Objects
| Field | Type | Description |
|-------|------|-------------|
| `has_next` | Boolean | Whether more pages exist |
| `page` | Integer | Current page number |
| `results` | Array | Array of entries for this page |

## Usage Examples

### Get first page of standings
```bash
curl "https://fantasy.premierleague.com/api/leagues-classic/314/standings/"
```

### Get second page of standings  
```bash
curl "https://fantasy.premierleague.com/api/leagues-classic/314/standings/?page_standings=2"
```

### Get specific phase standings
```bash
curl "https://fantasy.premierleague.com/api/leagues-classic/314/standings/?phase=1"
```

### Combine parameters
```bash
curl "https://fantasy.premierleague.com/api/leagues-classic/314/standings/?page_standings=1&phase=1"
```

## Popular League IDs

| League ID | Description |
|-----------|-------------|
| 314 | Overall Global League |
| Various | Country-specific leagues |
| Various | Mini-leagues created by users |

## Implementation Notes

1. **Pagination**: Use `has_next` to determine if more pages exist
2. **League Discovery**: Find league IDs from manager's league memberships via `/entry/{id}/`
3. **Rank Changes**: Compare `rank` vs `last_rank` for movement indicators  
4. **Manager Details**: Use `entry` ID with `/entry/{id}/` for full manager info

## Rate Limiting
Large leagues may have many pages - implement appropriate delays when paginating through results.

## Related Endpoints
- `/entry/{id}/` - Get manager details using `entry` field
- `/leagues-h2h-matches/league/{id}/` - Head-to-head league alternative
- `/bootstrap-static/` - Get overall league information
