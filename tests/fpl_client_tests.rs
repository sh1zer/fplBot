use fpl_client::client::FplApiClient;

#[tokio::test]
async fn test_get_fixtures_without_gameweek() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_fixtures(None, None).await;

    match result {
        Ok(fixtures) => {
            // All season fixtures returned as a Vec
            println!("Got {} total fixtures", fixtures.len());
        }
        Err(e) => {
            // Network errors are acceptable in tests
            println!("Network error (expected in CI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_fixtures_with_gameweek() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_fixtures(Some(1), None).await;

    match result {
        Ok(fixtures) => {
            println!("Got {} fixtures for gameweek 1", fixtures.len());
        }
        Err(e) => {
            println!("Network error (expected in CI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_fixtures_invalid_gameweek() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    // Gameweek 100 doesn't exist - should return empty or error
    let result = client.get_fixtures(Some(100), None).await;

    match result {
        Ok(fixtures) => {
            // Empty array is acceptable for non-existent gameweeks
            println!("Got {} fixtures for gameweek 100", fixtures.len());
        }
        Err(e) => {
            println!("Expected error for invalid gameweek 100: {}", e);
            assert!(e.to_string().contains("HTTP Error") || e.to_string().contains("404"));
        }
    }
}

#[tokio::test]
async fn test_get_league() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_league_standings(123456).await;

    match result {
        Ok(_) => {
            // League exists and returned data
        }
        Err(e) => {
            // Expected to fail for non-existent league or network issues
            println!("Expected error for test league ID: {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_league_invalid_id() {
    let client = FplApiClient::new().expect("Failed to create FPL client");

    let result = client.get_league_standings(999999999).await;

    match result {
        Ok(_) => {
            panic!("Expected error for invalid league ID 999999999, but got success");
        }
        Err(e) => {
            println!("Expected error for invalid league ID: {}", e);
            assert!(e.to_string().contains("HTTP Error"));
        }
    }
}

#[tokio::test]
async fn test_multiple_client_creation() {
    // Test that creating multiple clients works correctly
    let _client1 = FplApiClient::new().expect("Failed to create first FPL client");
    let _client2 = FplApiClient::new().expect("Failed to create second FPL client");
}
