use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::from_value;

use crate::utils::deserializers::de_f64_from_string;
use crate::fpl::fpl_client;

// results for endpoint event/{event_id}/live

/// Represents the complete live gameweek data response from the FPL API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameweekResponse {
    pub elements: Vec<PlayerGameweekStats>,
}

/// Contains live statistics and points for a single player during a gameweek.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGameweekStats {
    pub id: i32,

    pub stats: PlayerStats,

    pub explain: Vec<PlayerPointsExplanation>,

    pub modified: bool,
}

/// Contains all live statistical data for a player in a gameweek.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub minutes: i32,

    pub goals_scored: i32,

    pub assists: i32,

    pub clean_sheets: i32,

    pub goals_conceded: i32,

    pub own_goals: i32,

    pub penalties_saved: i32,

    pub penalties_missed: i32,

    pub yellow_cards: i32,

    pub red_cards: i32,

    pub saves: i32,

    pub bonus: i32,

    pub bps: i32,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub influence: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub creativity: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub threat: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub ict_index: f64,

    pub clearances_blocks_interceptions: i32,

    pub recoveries: i32,

    pub tackles: i32,

    pub defensive_contribution: i32,

    pub starts: i32,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub expected_goals: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub expected_assists: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub expected_goal_involvements: f64,

    #[serde(deserialize_with = "de_f64_from_string")]
    pub expected_goals_conceded: f64,

    pub total_points: i32,

    pub in_dreamteam: bool,
}

/// Explains how a player's points were calculated for specific fixtures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPointsExplanation {
    pub fixture: i32,

    pub stats: Vec<StatPointsBreakdown>,
}

/// Detailed breakdown of points awarded for a specific statistic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatPointsBreakdown {
    pub identifier: String,

    pub points: i32,

    pub value: i32,

    pub points_modification: i32,
}


pub async fn update_gameweek_info(gameweek: i32) -> Result<GameweekResponse>{
    let response = fpl_client().get_gameweek(gameweek).await?;

    Ok(from_value(response)?)
}

