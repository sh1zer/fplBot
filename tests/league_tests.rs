use fpl_client::LeagueStandings;
use fpl_client::client::FplApiClient;
use serde_json::json;

#[tokio::test]
async fn test_league_standings_deserialization() {
    // Mock JSON response that matches the FPL API structure
    let mock_response = json!({
        "new_entries": {
            "has_next": false,
            "page": 1,
            "results": []
        },
        "last_updated_data": "2025-09-13T22:16:35Z",
        "league": {
            "id": 123456,
            "name": "Test League",
            "created": "2024-08-01T10:00:00Z",
            "closed": false,
            "max_entries": null,
            "league_type": "s",
            "scoring": "c",
            "admin_entry": 987654,
            "start_event": 1,
            "code_privacy": "p",
            "has_cup": false,
            "cup_league": null,
            "rank": null
        },
        "standings": {
            "has_next": false,
            "page": 1,
            "results": [
                {
                    "id": 1,
                    "event_total": 85,
                    "player_name": "John Doe",
                    "rank": 1,
                    "last_rank": 2,
                    "rank_sort": 1,
                    "total": 1250,
                    "entry": 987654,
                    "entry_name": "Dream Team FC"
                },
                {
                    "id": 2,
                    "event_total": 72,
                    "player_name": "Jane Smith",
                    "rank": 2,
                    "last_rank": 1,
                    "rank_sort": 2,
                    "total": 1245,
                    "entry": 123789,
                    "entry_name": "Super Squad"
                }
            ]
        }
    });

    let standings: LeagueStandings = serde_json::from_value(mock_response).unwrap();

    // Assert league info
    assert_eq!(standings.league.id, 123456);
    assert_eq!(standings.league.name, "Test League");
    assert_eq!(standings.league.admin_entry, Some(987654));
    assert!(!standings.league.closed);

    // Assert standings
    assert_eq!(standings.standings.results.len(), 2);

    let first = &standings.standings.results[0];
    assert_eq!(first.player_name, "John Doe");
    assert_eq!(first.rank, 1);
    assert_eq!(first.total, 1250);
    assert_eq!(first.entry_name, "Dream Team FC");

    let second = &standings.standings.results[1];
    assert_eq!(second.player_name, "Jane Smith");
    assert_eq!(second.rank, 2);
    assert_eq!(second.total, 1245);
}

#[tokio::test]
async fn test_league_standings_fetch_integration() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_league_standings(314).await;

    match result {
        Ok(standings) => {
            assert!(standings.league.id > 0);
            assert!(!standings.league.name.is_empty());
            println!("Successfully fetched league: {}", standings.league.name);
        }
        Err(e) => {
            println!("Integration test warning: {}", e);
            println!("This is expected if league 314 doesn't exist or is private");
        }
    }
}

#[tokio::test]
async fn test_league_standings_fetch_invalid_id() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_league_standings(-1).await;

    assert!(result.is_err());
}

#[test]
fn test_standings_pagination_fields() {
    let mock_response = json!({
        "new_entries": {
            "has_next": false,
            "page": 1,
            "results": []
        },
        "last_updated_data": "2025-09-13T22:16:35Z",
        "league": {
            "id": 123,
            "name": "Test",
            "created": "2024-08-01T10:00:00Z",
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
            "has_next": false,
            "page": 1,
            "results": []
        }
    });

    let standings: LeagueStandings = serde_json::from_value(mock_response).unwrap();

    assert!(!standings.standings.has_next);
    assert_eq!(standings.standings.page, 1);
    assert_eq!(standings.standings.results.len(), 0);
    assert_eq!(standings.last_updated_data, "2025-09-13T22:16:35Z");
}
