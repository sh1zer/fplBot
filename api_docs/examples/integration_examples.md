# FPL API Integration Examples

This document provides practical code examples for integrating the discovered FPL API capabilities into your bot.

## Key Parameter Usage

### 1. Fixtures Filtering - The Most Useful Feature

```rust
use reqwest::Client;
use serde_json::Value;

// Get fixtures for current gameweek
async fn get_current_gameweek_fixtures(client: &Client, gameweek: u32) -> Result<Vec<Value>, reqwest::Error> {
    let url = format!("https://fantasy.premierleague.com/api/fixtures/?event={}", gameweek);
    let response = client.get(&url).send().await?;
    let fixtures: Vec<Value> = response.json().await?;
    Ok(fixtures) // Returns ~10 fixtures
}

// Get all fixtures for a specific team
async fn get_team_fixtures(client: &Client, team_id: u32) -> Result<Vec<Value>, reqwest::Error> {
    let url = format!("https://fantasy.premierleague.com/api/fixtures/?team={}", team_id);
    let response = client.get(&url).send().await?;
    let fixtures: Vec<Value> = response.json().await?;
    Ok(fixtures) // Returns 38 fixtures for that team
}

// Find upcoming fixtures for a team
async fn get_upcoming_team_fixtures(client: &Client, team_id: u32) -> Result<Vec<Value>, reqwest::Error> {
    let fixtures = get_team_fixtures(client, team_id).await?;
    let upcoming: Vec<Value> = fixtures.into_iter()
        .filter(|fixture| {
            fixture.get("finished").and_then(|f| f.as_bool()).unwrap_or(false) == false
        })
        .collect();
    Ok(upcoming)
}
```

### 2. League Standings Pagination

```rust
// Get all standings for a league (handles pagination)
async fn get_complete_league_standings(client: &Client, league_id: u32) -> Result<Vec<Value>, reqwest::Error> {
    let mut all_standings = Vec::new();
    let mut page = 1;
    
    loop {
        let url = format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/?page_standings={}", league_id, page);
        let response = client.get(&url).send().await?;
        let data: Value = response.json().await?;
        
        if let Some(standings) = data.get("standings") {
            if let Some(results) = standings.get("results").and_then(|r| r.as_array()) {
                all_standings.extend(results.clone());
            }
            
            let has_next = standings.get("has_next").and_then(|h| h.as_bool()).unwrap_or(false);
            if !has_next {
                break;
            }
        } else {
            break;
        }
        
        page += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; // Be nice to API
    }
    
    Ok(all_standings)
}
```

### 3. Current Gameweek Detection Using Events Endpoint

```rust
// Get current gameweek info using the discovered /events/ endpoint
async fn get_current_gameweek_info(client: &Client) -> Result<Option<Value>, reqwest::Error> {
    let url = "https://fantasy.premierleague.com/api/events/";
    let response = client.get(&url).send().await?;
    let events: Vec<Value> = response.json().await?;
    
    let current_gameweek = events.into_iter()
        .find(|event| {
            event.get("is_current").and_then(|c| c.as_bool()).unwrap_or(false)
        });
    
    Ok(current_gameweek)
}

// Get gameweek deadline and average score
async fn get_gameweek_stats(client: &Client, gameweek_id: u32) -> Result<Option<(String, u32)>, reqwest::Error> {
    let url = "https://fantasy.premierleague.com/api/events/";
    let response = client.get(&url).send().await?;
    let events: Vec<Value> = response.json().await?;
    
    if let Some(event) = events.into_iter().find(|e| e.get("id").and_then(|i| i.as_u64()) == Some(gameweek_id as u64)) {
        let deadline = event.get("deadline_time").and_then(|d| d.as_str()).unwrap_or("").to_string();
        let avg_score = event.get("average_entry_score").and_then(|s| s.as_u64()).unwrap_or(0) as u32;
        return Ok(Some((deadline, avg_score)));
    }
    
    Ok(None)
}
```

## Bot Command Examples

### Discord Bot Commands Using Discovered Parameters

```rust
// Bot command: !fixtures arsenal
async fn fixtures_command(ctx: &Context, msg: &Message, team_name: &str) {
    let client = reqwest::Client::new();
    
    // First get team ID from bootstrap-static
    let bootstrap_url = "https://fantasy.premierleague.com/api/bootstrap-static/";
    let bootstrap: Value = client.get(bootstrap_url).send().await.unwrap().json().await.unwrap();
    
    let team_id = bootstrap.get("teams")
        .and_then(|teams| teams.as_array())
        .and_then(|teams| {
            teams.iter().find(|team| {
                team.get("name").and_then(|n| n.as_str())
                    .map(|name| name.to_lowercase().contains(&team_name.to_lowercase()))
                    .unwrap_or(false)
            })
        })
        .and_then(|team| team.get("id").and_then(|id| id.as_u64()));
    
    if let Some(team_id) = team_id {
        // Use the discovered team parameter!
        let fixtures = get_upcoming_team_fixtures(&client, team_id as u32).await.unwrap();
        
        let mut response = format!("📅 Upcoming fixtures for {}:\n", team_name);
        for fixture in fixtures.iter().take(5) {
            let kickoff = fixture.get("kickoff_time").and_then(|k| k.as_str()).unwrap_or("");
            let home_team = fixture.get("team_h").and_then(|t| t.as_u64()).unwrap_or(0);
            let away_team = fixture.get("team_a").and_then(|t| t.as_u64()).unwrap_or(0);
            
            response.push_str(&format!("• {} vs {} - {}\n", home_team, away_team, kickoff));
        }
        
        let _ = msg.channel_id.say(&ctx.http, response).await;
    } else {
        let _ = msg.channel_id.say(&ctx.http, "Team not found!").await;
    }
}

// Bot command: !gameweek current  
async fn current_gameweek_command(ctx: &Context, msg: &Message) {
    let client = reqwest::Client::new();
    
    if let Ok(Some(current_gw)) = get_current_gameweek_info(&client).await {
        let gw_num = current_gw.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
        let deadline = current_gw.get("deadline_time").and_then(|d| d.as_str()).unwrap_or("");
        let avg_score = current_gw.get("average_entry_score").and_then(|s| s.as_u64()).unwrap_or(0);
        
        let response = format!(
            "📊 Gameweek {} Status:\n• Deadline: {}\n• Average Score: {}\n• Finished: {}",
            gw_num,
            deadline,
            avg_score,
            current_gw.get("finished").and_then(|f| f.as_bool()).unwrap_or(false)
        );
        
        let _ = msg.channel_id.say(&ctx.http, response).await;
    }
}

// Bot command: !league 314 (using pagination)
async fn league_command(ctx: &Context, msg: &Message, league_id: u32) {
    let client = reqwest::Client::new();
    
    // Get first page only for quick response
    let url = format!("https://fantasy.premierleague.com/api/leagues-classic/{}/standings/?page_standings=1", league_id);
    let response = client.get(&url).send().await.unwrap();
    let data: Value = response.json().await.unwrap();
    
    if let Some(standings) = data.get("standings").and_then(|s| s.get("results")).and_then(|r| r.as_array()) {
        let league_name = data.get("league").and_then(|l| l.get("name")).and_then(|n| n.as_str()).unwrap_or("League");
        
        let mut response_msg = format!("🏆 {} Top 10:\n", league_name);
        for (i, entry) in standings.iter().take(10).enumerate() {
            let rank = entry.get("rank").and_then(|r| r.as_u64()).unwrap_or(i as u64 + 1);
            let name = entry.get("player_name").and_then(|n| n.as_str()).unwrap_or("Unknown");
            let points = entry.get("total").and_then(|p| p.as_u64()).unwrap_or(0);
            
            response_msg.push_str(&format!("{}. {} - {} pts\n", rank, name, points));
        }
        
        let _ = msg.channel_id.say(&ctx.http, response_msg).await;
    }
}
```

## Caching Strategy

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct ApiCache {
    bootstrap_cache: Option<(Value, Instant)>,
    events_cache: Option<(Vec<Value>, Instant)>,
    fixtures_cache: HashMap<String, (Vec<Value>, Instant)>,
}

impl ApiCache {
    fn new() -> Self {
        Self {
            bootstrap_cache: None,
            events_cache: None,
            fixtures_cache: HashMap::new(),
        }
    }
    
    // Cache bootstrap data for 1 hour
    async fn get_bootstrap_data(&mut self, client: &Client) -> Result<Value, reqwest::Error> {
        if let Some((data, cached_at)) = &self.bootstrap_cache {
            if cached_at.elapsed() < Duration::from_secs(3600) {
                return Ok(data.clone());
            }
        }
        
        let url = "https://fantasy.premierleague.com/api/bootstrap-static/";
        let data: Value = client.get(url).send().await?.json().await?;
        self.bootstrap_cache = Some((data.clone(), Instant::now()));
        Ok(data)
    }
    
    // Cache events data for 30 minutes
    async fn get_events_data(&mut self, client: &Client) -> Result<Vec<Value>, reqwest::Error> {
        if let Some((data, cached_at)) = &self.events_cache {
            if cached_at.elapsed() < Duration::from_secs(1800) {
                return Ok(data.clone());
            }
        }
        
        let url = "https://fantasy.premierleague.com/api/events/";
        let data: Vec<Value> = client.get(url).send().await?.json().await?;
        self.events_cache = Some((data.clone(), Instant::now()));
        Ok(data)
    }
    
    // Cache fixtures by key for 15 minutes  
    async fn get_fixtures(&mut self, client: &Client, cache_key: &str, url: &str) -> Result<Vec<Value>, reqwest::Error> {
        if let Some((data, cached_at)) = self.fixtures_cache.get(cache_key) {
            if cached_at.elapsed() < Duration::from_secs(900) {
                return Ok(data.clone());
            }
        }
        
        let data: Vec<Value> = client.get(url).send().await?.json().await?;
        self.fixtures_cache.insert(cache_key.to_string(), (data.clone(), Instant::now()));
        Ok(data)
    }
}
```

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FplApiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Team not found: {0}")]
    TeamNotFound(String),
    
    #[error("League not found: {0}")]
    LeagueNotFound(u32),
    
    #[error("Gameweek not found: {0}")]
    GameweekNotFound(u32),
    
    #[error("API response parsing failed")]
    ParseError,
}

// Robust fixture fetching with error handling
async fn get_team_fixtures_safe(client: &Client, team_id: u32) -> Result<Vec<Value>, FplApiError> {
    let url = format!("https://fantasy.premierleague.com/api/fixtures/?team={}", team_id);
    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(FplApiError::TeamNotFound(team_id.to_string()));
    }
    
    let fixtures: Vec<Value> = response.json().await?;
    
    if fixtures.is_empty() {
        return Err(FplApiError::TeamNotFound(team_id.to_string()));
    }
    
    Ok(fixtures)
}
```

These examples show how to leverage the discovered API parameters effectively in your bot, with proper error handling and caching strategies.