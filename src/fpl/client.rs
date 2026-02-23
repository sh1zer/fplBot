use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use log::{error, info};

use std::sync::OnceLock;

static FPL_CLIENT: OnceLock<FplApiClient> = OnceLock::new();

/// Initializes the global FPL API service singleton.
/// 
/// This function must be called once at application startup before any 
/// calls to `fpl_client()`. It creates a new `FplApiClient` instance and 
/// stores it in a global static for application-wide access.
/// 
/// # Returns
/// 
/// * `Ok(())` - Service initialized successfully
/// * `Err` - Service already initialized or client creation failed
/// 
/// # Examples
/// 
/// ```
/// use fplbot::fpl::client::init_fpl_service;
/// 
/// init_fpl_service().expect("Failed to initialize FPL service");
/// ```
pub fn init_fpl_service() -> Result<()> {
    let client = FplApiClient::new();
    FPL_CLIENT.set(client)
        .map_err(|_| anyhow::anyhow!("FPL service already initialized"))?;
    Ok(())
}

/// Returns a reference to the global FPL API client instance.
/// 
/// This function provides access to the singleton `FplApiClient` that was 
/// initialized by `init_fpl_service()`. The client can be used to make 
/// requests to the Fantasy Premier League API endpoints.
/// 
/// # Panics
/// 
/// Panics if `init_fpl_service()` has not been called first.
/// 
/// # Returns
/// 
/// A static reference to the `FplApiClient` instance.
/// 
/// # Examples
/// 
/// ```
/// use fplbot::fpl::client::fpl_client;
/// 
/// let client = fpl_client();
/// let general_data = client.get_general().await?;
/// ```
pub fn fpl_client() -> &'static FplApiClient {
    FPL_CLIENT.get()
        .expect("FPL service not initialized - call init_fpl_service() first")
}

#[derive(Debug)]
pub struct FplApiClient {
    client: Client,
    base_url: String,
}

impl FplApiClient {
    fn new() -> Self {
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(300))
            .pool_max_idle_per_host(2)
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://fantasy.premierleague.com/api".to_string(),
        }
    }

    /// Returns the base URL for the Fantasy Premier League API.
    /// 
    /// # Returns
    /// 
    /// A string slice containing the FPL API base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn _get_request(&self, endpoint: impl Into<String>, params: Option<HashMap<String, String>>) -> Result<Value> {
        let url = format!("{}/{}/", self.base_url, endpoint.into());
        info!("Making GET request to {} with params {:?}", url, params);

        let mut request = self.client.get(&url);
        
        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            error!("HTTP Error: {} for URL {}", response.status(), url);
            return Err(anyhow::anyhow!("HTTP Error: {}", response.status()));
        }

        let json = response.json::<Value>().await?;
        Ok(json)
    }

    /// Fetches general FPL data including teams, players, and gameweek information.
    /// 
    /// This endpoint provides the main bootstrap data for the FPL API, containing
    /// information about all teams, players, current gameweek, and other static data.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing general FPL data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let general_data = fpl_client().get_general().await?;
    /// ```
    pub async fn get_general(&self) -> Result<Value>{
        self._get_request("bootstrap-static", None).await
    }

    /// Fetches fixture data for a specific gameweek or all fixtures.
    /// 
    /// # Parameters
    /// 
    /// * `gameweek` - Optional gameweek number. If `None`, returns all fixtures.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing fixture data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Get all fixtures
    /// let all_fixtures = fpl_client().get_fixtures(None).await?;
    /// 
    /// // Get fixtures for gameweek 10
    /// let gw10_fixtures = fpl_client().get_fixtures(Some(10)).await?;
    /// ```
    pub async fn get_fixtures(&self, gameweek: Option<i32>) -> Result<Value> {
        let params = gameweek.map(|gw| {
            let mut map = HashMap::new();
            map.insert("event".to_string(), gw.to_string());
            map
        });
        
        self._get_request("fixtures", params).await
    }

    /// Fetches league standings for a classic league.
    /// 
    /// This is a convenience method that calls `get_league_standings` with no page parameter.
    /// 
    /// # Parameters
    /// 
    /// * `league_id` - The ID of the classic league
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing league standings data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let standings = fpl_client().get_league(314).await?;
    /// ```
    pub async fn get_league(&self, league_id: i32) -> Result<Value> {
        self.get_league_standings(league_id, None).await
    }

    /// Fetches league standings for a classic league with optional pagination.
    /// 
    /// # Parameters
    /// 
    /// * `league_id` - The ID of the classic league
    /// * `page` - Optional page number for pagination (1-based). If `None`, returns first page.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing league standings data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Get first page of standings
    /// let standings = fpl_client().get_league_standings(314, None).await?;
    /// 
    /// // Get second page of standings
    /// let page2 = fpl_client().get_league_standings(314, Some(2)).await?;
    /// ```
    pub async fn get_league_standings(&self, league_id: i32, page: Option<i32>) -> Result<Value> {
        let params = page.map(|p| {
            let mut map = HashMap::new();
            map.insert("page_standings".to_string(), p.to_string());
            map
        });
        
        self._get_request(format!("leagues-classic/{}/standings", league_id), params).await
    }

    /// Fetches summary information for a specific FPL manager.
    /// 
    /// # Parameters
    /// 
    /// * `manager_id` - The FPL manager ID
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing manager summary data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let manager_data = fpl_client().get_manager_summary(123456).await?;
    /// ```
    pub async fn get_manager_summary(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}", manager_id), None).await
    }
    /// Fetches historical performance data for a specific FPL manager.
    /// 
    /// # Parameters
    /// 
    /// * `manager_id` - The FPL manager ID
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing manager's historical data
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let history = fpl_client().get_manager_history(123456).await?;
    /// ```
    pub async fn get_manager_history(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/history", manager_id), None).await
    }
    /// Fetches transfer history for a specific FPL manager.
    /// 
    /// # Parameters
    /// 
    /// * `manager_id` - The FPL manager ID
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing manager's transfer history
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let transfers = fpl_client().get_manager_transfers(123456).await?;
    /// ```
    pub async fn get_manager_transfers(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/transfers", manager_id), None).await
    }
    /// Fetches a manager's team selection for a specific gameweek.
    /// 
    /// # Parameters
    /// 
    /// * `manager_id` - The FPL manager ID
    /// * `gameweek` - The gameweek number
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing the manager's team picks
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let team_picks = fpl_client().get_manager_team(123456, 10).await?;
    /// ```
    pub async fn get_manager_team(&self, manager_id: i32, gameweek: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/event/{}/picks", manager_id, gameweek), None).await
    }

    /// Fetches detailed information for a specific player.
    /// 
    /// # Parameters
    /// 
    /// * `player_id` - The FPL player ID (element ID)
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing player's detailed statistics
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let player_stats = fpl_client().get_player_summary(123).await?;
    /// ```
    pub async fn get_player_summary(&self, player_id: i32) -> Result<Value> {
        self._get_request(format!("element-summary/{}", player_id), None).await
    }

    /// Fetches live data for a specific gameweek.
    /// 
    /// # Parameters
    /// 
    /// * `gameweek` - The gameweek number
    /// 
    /// # Returns
    /// 
    /// * `Ok(Value)` - JSON response containing live gameweek data including player scores
    /// * `Err` - Network error or API error
    /// 
    /// # Examples
    /// 
    /// ```
    /// let live_data = fpl_client().get_gameweek(10).await?;
    /// ```
    pub async fn get_gameweek(&self, gameweek: i32) -> Result<Value> {
        self._get_request(format!("event/{}/live", gameweek,), None).await
    }
}
