use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct FplApiClient {
    client: Client,
    base_url: String,
}

impl FplApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
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

    async fn get_request(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<Value> {
        let url = format!("{}/{}/", self.base_url, endpoint);
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

    pub async fn get_fixtures(&self, gameweek: Option<u32>) -> Result<Value> {
        let params = gameweek.map(|gw| {
            let mut map = HashMap::new();
            map.insert("event".to_string(), gw.to_string());
            map
        });
        
        self.get_request("fixtures", params).await
    }

    pub async fn get_league(&self, league_id: u32) -> Result<Value> {
        let endpoint = format!("leagues-classic/{}/standings", league_id);
        self.get_request(&endpoint, None).await
    }
}