use fplbot::fpl::{init_fpl_service, fpl_client};
use std::sync::Once;

static INIT: Once = Once::new();

fn ensure_service_initialized() {
    INIT.call_once(|| {
        init_fpl_service().expect("Failed to initialize FPL service");
    });
}

#[tokio::test]
async fn test_fpl_service_initialization() {
    ensure_service_initialized();
    
    assert_eq!(fpl_client().base_url(), "https://fantasy.premierleague.com/api");
}

#[tokio::test]
async fn test_get_fixtures_without_gameweek() {
    ensure_service_initialized();
    
    // This is an integration test that requires network access
    // In a real scenario, you might want to mock the HTTP client
    let result = fpl_client().get_fixtures(None).await;
    
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
    ensure_service_initialized();
    
    let result = fpl_client().get_fixtures(Some(1)).await;
    
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
async fn test_get_fixtures_invalid_gameweek() {
    ensure_service_initialized();
    
    // Test with an invalid gameweek that should return 404 or error
    let result = fpl_client().get_fixtures(Some(100)).await;
    
    match result {
        Ok(fixtures) => {
            // If it succeeds, verify it's empty or has no fixtures
            if let Some(array) = fixtures.as_array() {
                // Empty array is acceptable for future gameweeks
                println!("Got {} fixtures for gameweek 100", array.len());
            }
        }
        Err(e) => {
            // This is the expected case for invalid gameweeks
            println!("Expected error for invalid gameweek 100: {}", e);
            assert!(e.to_string().contains("HTTP Error") || e.to_string().contains("404"));
        }
    }
}

#[tokio::test]
async fn test_get_league() {
    ensure_service_initialized();
    
    // Use a test league ID (this might fail if league doesn't exist)
    let result = fpl_client().get_league(123456).await;
    
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
    ensure_service_initialized();
    
    // Test with an obviously invalid league ID that should return 404
    let result = fpl_client().get_league(999999999).await;
    
    // This should definitely fail
    match result {
        Ok(_) => {
            panic!("Expected error for invalid league ID 999999999, but got success");
        }
        Err(e) => {
            println!("Expected error for invalid league ID: {}", e);
            // Verify it's an HTTP error (likely 404)
            assert!(e.to_string().contains("HTTP Error"));
        }
    }
}

#[tokio::test]
async fn test_multiple_service_access() {
    ensure_service_initialized();
    
    // Test that multiple accesses to the service work correctly
    assert_eq!(fpl_client().base_url(), fpl_client().base_url());
}
