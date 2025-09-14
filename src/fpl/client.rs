use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, time::Duration};
use tracing::{error, info};

use std::sync::OnceLock;

static FPL_CLIENT: OnceLock<FplApiClient> = OnceLock::new();

pub fn init_fpl_service() -> Result<()> {
    let client = FplApiClient::new();
    FPL_CLIENT.set(client)
        .map_err(|_| anyhow::anyhow!("FPL service already initialized"))?;
    Ok(())
}

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

    pub async fn get_general(&self) -> Result<Value>{
        self._get_request("bootstrap-static", None).await
    }

    pub async fn get_fixtures(&self, gameweek: Option<i32>) -> Result<Value> {
        let params = gameweek.map(|gw| {
            let mut map = HashMap::new();
            map.insert("event".to_string(), gw.to_string());
            map
        });
        
        self._get_request("fixtures", params).await
    }

    pub async fn get_league(&self, league_id: i32) -> Result<Value> {
        self.get_league_standings(league_id, None).await
    }

    pub async fn get_league_standings(&self, league_id: i32, page: Option<i32>) -> Result<Value> {
        let params = page.map(|p| {
            let mut map = HashMap::new();
            map.insert("page_standings".to_string(), p.to_string());
            map
        });
        
        self._get_request(format!("leagues-classic/{}/standings", league_id), params).await
    }

    pub async fn get_manager_summary(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}", manager_id), None).await
    }
    pub async fn get_manager_history(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/history", manager_id), None).await
    }
    pub async fn get_manager_transfers(&self, manager_id: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/transfers", manager_id), None).await
    }
    pub async fn get_manager_team(&self, manager_id: i32, gameweek: i32) -> Result<Value> {
        self._get_request(format!("entry/{}/event/{}/picks", manager_id, gameweek), None).await
    }

    pub async fn get_player_summary(&self, player_id: i32) -> Result<Value> {
        self._get_request(format!("element-summary/{}", player_id), None).await
    }

    pub async fn get_gameweek(&self, gameweek: i32) -> Result<Value> {
        self._get_request(format!("event/{}/live", gameweek,), None).await
    }
}
