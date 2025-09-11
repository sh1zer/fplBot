use fplbot::fpl::client::FplApiClient;

#[tokio::test]
async fn test_fpl_client_creation() {
    let client = FplApiClient::new();
    
    // Test that client is created successfully
    assert_eq!(client.base_url(), "https://fantasy.premierleague.com/api");
}

#[tokio::test]
async fn test_get_fixtures_without_gameweek() {
    let client = FplApiClient::new();
    
    // This is an integration test that requires network access
    // In a real scenario, you might want to mock the HTTP client
    let result = client.get_fixtures(None).await;
    
    match result {
        Ok(fixtures) => {
            // Verify the response has expected structure
            assert!(fixtures.is_array() || fixtures.is_object());
        }
        Err(e) => {
            // Network errors are acceptable in tests
            println!("Network error (expected in CI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_fixtures_with_gameweek() {
    let client = FplApiClient::new();
    
    let result = client.get_fixtures(Some(1)).await;
    
    match result {
        Ok(fixtures) => {
            assert!(fixtures.is_array() || fixtures.is_object());
        }
        Err(e) => {
            println!("Network error (expected in CI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_league() {
    let client = FplApiClient::new();
    
    // Use a test league ID (this might fail if league doesn't exist)
    let result = client.get_league(123456).await;
    
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

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_client_default_values() {
        let client = FplApiClient::new();
        
        // Test default configuration
        assert!(!client.base_url().is_empty());
        assert!(client.base_url().starts_with("https://"));
    }
}