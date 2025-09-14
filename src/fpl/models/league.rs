use serde::{Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use serde_json::from_value;

use crate::fpl::fpl_client;

#[derive(Debug, Deserialize)]
pub struct LeagueStandings {
    #[serde(rename = "new_entries")]
    pub new_managers: NewManagers,
    #[serde(rename = "last_updated_data")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "league")]
    pub league_info: LeagueInfo,
    pub standings: Standings,
}

#[derive(Debug, Deserialize)]
pub struct NewManagers {
    pub has_next: bool,
    pub page: i32,
    #[serde(rename = "results")]
    pub managers: Vec<LeagueManager>,
}

#[derive(Debug, Deserialize)]
pub struct LeagueInfo {
    pub id: i32,
    #[serde(rename = "name")]
    pub league_name: String,
    #[serde(rename = "created")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "max_entries")]
    pub max_managers: Option<i32>,
    #[serde(rename = "league_type")]
    pub scoring_type: String,
    #[serde(rename = "scoring")]
    pub scoring_method: String,
    #[serde(rename = "admin_entry")]
    pub admin_manager_id: Option<i32>,
    #[serde(rename = "start_event")]
    pub start_gameweek: i32,
    #[serde(rename = "code_privacy")]
    pub privacy_setting: String,
    #[serde(rename = "has_cup")]
    pub has_cup_competition: bool,
    #[serde(rename = "cup_league")]
    pub cup_league_id: Option<i32>,
    #[serde(rename = "rank")]
    pub league_rank: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Standings {
    pub has_next: bool,
    pub page: i32,
    #[serde(rename = "results")]
    pub managers: Vec<StandingsManager>,
}

#[derive(Debug, Deserialize)]
pub struct LeagueManager {
    pub id: i32,
    #[serde(rename = "name")]
    pub league_name: String,
    #[serde(rename = "created")]
    pub created_date: DateTime<Utc>,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "admin_entry")]
    pub admin_manager_id: i32,
    #[serde(rename = "start_event")]
    pub start_gameweek: i32,
    #[serde(rename = "entry_can_leave")]
    pub can_leave: bool,
    #[serde(rename = "entry_can_admin")]
    pub can_admin: bool,
    #[serde(rename = "entry_can_invite")]
    pub can_invite: bool,
    #[serde(rename = "rank_count")]
    pub total_managers: i32,
    #[serde(rename = "entry_percentile_rank")]
    pub percentile_rank: i32,
    #[serde(rename = "entry_rank")]
    pub current_rank: i32,
    #[serde(rename = "entry_last_rank")]
    pub previous_rank: i32,
}

#[derive(Debug, Deserialize)]
pub struct StandingsManager {
    pub id: i32,
    #[serde(rename = "event_total")]
    pub gameweek_points: i32,
    #[serde(rename = "player_name")]
    pub manager_name: String,
    #[serde(rename = "rank")]
    pub current_rank: i32,
    #[serde(rename = "last_rank")]
    pub previous_rank: i32,
    #[serde(rename = "rank_sort")]
    pub sort_rank: i32,
    #[serde(rename = "total")]
    pub total_points: i32,
    #[serde(rename = "entry")]
    pub manager_id: i32,
    #[serde(rename = "entry_name")]
    pub team_name: String,
    #[serde(rename = "has_played")]
    pub has_played: bool,
}

impl LeagueStandings {
    pub async fn fetch(league_id: i32) -> Result<Self>{
        let response = fpl_client().get_league(league_id).await?;
        Ok(from_value(response)?)
    }


}
