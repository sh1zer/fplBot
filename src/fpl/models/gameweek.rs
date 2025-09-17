use serde::{Deserialize, Serialize};

// results for endpoint event/{event_id}/live

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameweekResponse {
    pub elements: Vec<PlayerGameweekStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGameweekStats {
    pub id: i32,
    pub stats: PlayerStats,
    pub explain: Vec<PlayerPointsExplanation>,
    pub modified: bool,
}

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
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub clearances_blocks_interceptions: i32,
    pub recoveries: i32,
    pub tackles: i32,
    pub defensive_contribution: i32,
    pub starts: i32,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub expected_goals_conceded: String,
    pub total_points: i32,
    pub in_dreamteam: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPointsExplanation {
    pub fixture: i32,
    pub stats: Vec<StatPointsBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatPointsBreakdown {
    pub identifier: String,
    pub points: i32,
    pub value: i32,
    pub points_modification: i32,
}
