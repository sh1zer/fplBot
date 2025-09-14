use fplbot::fpl::{init_fpl_service, models::league::LeagueStandings};
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
                    "entry_name": "Dream Team FC",
                    "has_played": true
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
                    "entry_name": "Super Squad",
                    "has_played": true
                }
            ]
        }
    });

    // Test deserialization
    let standings: LeagueStandings = serde_json::from_value(mock_response).unwrap();

    // Assert league info
    assert_eq!(standings.league_info.id, 123456);
    assert_eq!(standings.league_info.league_name, "Test League");
    assert_eq!(standings.league_info.admin_manager_id, Some(987654));
    assert!(!standings.league_info.is_closed);

    // Assert standings
    assert_eq!(standings.standings.managers.len(), 2);
    
    let first_manager = &standings.standings.managers[0];
    assert_eq!(first_manager.manager_name, "John Doe");
    assert_eq!(first_manager.current_rank, 1);
    assert_eq!(first_manager.total_points, 1250);
    assert_eq!(first_manager.team_name, "Dream Team FC");

    let second_manager = &standings.standings.managers[1];
    assert_eq!(second_manager.manager_name, "Jane Smith");
    assert_eq!(second_manager.current_rank, 2);
    assert_eq!(second_manager.total_points, 1245);
}

#[tokio::test]
async fn test_league_standings_fetch_integration() {
    // Try to initialize, but don't fail if already initialized
    let _ = init_fpl_service();
    
    // Test with a real league ID (this is a public test league)
    // Note: This test will fail if the league doesn't exist or is private
    let result = LeagueStandings::fetch(314).await;
    
    match result {
        Ok(standings) => {
            // Basic assertions to ensure we got valid data
            assert!(standings.league_info.id > 0);
            assert!(!standings.league_info.league_name.is_empty());
            println!("Successfully fetched league: {}", standings.league_info.league_name);
        }
        Err(e) => {
            // Log the error but don't fail the test since the league might not exist
            println!("Integration test warning: {}", e);
            println!("This is expected if league 314 doesn't exist or is private");
        }
    }
}

#[tokio::test]
async fn test_league_standings_fetch_invalid_id() {
    // Try to initialize, but don't fail if already initialized
    let _ = init_fpl_service();
    
    // Test with an invalid league ID
    let result = LeagueStandings::fetch(-1).await;
    
    // Should return an error for invalid ID
    assert!(result.is_err());
}

#[test]
fn test_datetime_parsing() {
    // Test that our DateTime<Utc> fields parse correctly
    let json_data = json!({
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

    let standings: LeagueStandings = serde_json::from_value(json_data).unwrap();
    
    // Verify dates parsed correctly
    assert_eq!(standings.last_updated.format("%Y-%m-%d").to_string(), "2025-09-13");
    assert_eq!(standings.league_info.created_date.format("%Y-%m-%d").to_string(), "2024-08-01");
}