use serde::{Deserialize, Serialize};

// results for endpoint event/{event_id}/live

/// Represents the complete live gameweek data response from the FPL API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameweekResponse {
    /// List of all players and their live stats for this gameweek
    pub elements: Vec<PlayerGameweekStats>,
}

/// Contains live statistics and points for a single player during a gameweek.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGameweekStats {
    /// Player ID (element ID)
    pub id: i32,
    /// Player's live statistics
    pub stats: PlayerStats,
    /// Detailed breakdown of how points were awarded
    pub explain: Vec<PlayerPointsExplanation>,
    /// Whether this player's data has been modified since last update
    pub modified: bool,
}

/// Contains all live statistical data for a player in a gameweek.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Minutes played
    pub minutes: i32,
    /// Goals scored
    pub goals_scored: i32,
    /// Assists provided
    pub assists: i32,
    /// Clean sheets achieved (defenders/goalkeepers)
    pub clean_sheets: i32,
    /// Goals conceded (defenders/goalkeepers)
    pub goals_conceded: i32,
    /// Own goals scored
    pub own_goals: i32,
    /// Penalties saved (goalkeepers)
    pub penalties_saved: i32,
    /// Penalties missed
    pub penalties_missed: i32,
    /// Yellow cards received
    pub yellow_cards: i32,
    /// Red cards received
    pub red_cards: i32,
    /// Saves made (goalkeepers)
    pub saves: i32,
    /// Bonus points awarded
    pub bonus: i32,
    /// Bonus Points System score
    pub bps: i32,
    /// ICT Influence score (as string)
    pub influence: String,
    /// ICT Creativity score (as string)
    pub creativity: String,
    /// ICT Threat score (as string)
    pub threat: String,
    /// Combined ICT Index score (as string)
    pub ict_index: String,
    /// Clearances, blocks, and interceptions
    pub clearances_blocks_interceptions: i32,
    /// Ball recoveries
    pub recoveries: i32,
    /// Tackles made
    pub tackles: i32,
    /// Overall defensive contribution
    pub defensive_contribution: i32,
    /// Games started
    pub starts: i32,
    /// Expected goals (as string)
    pub expected_goals: String,
    /// Expected assists (as string)
    pub expected_assists: String,
    /// Expected goal involvements (as string)
    pub expected_goal_involvements: String,
    /// Expected goals conceded (as string)
    pub expected_goals_conceded: String,
    /// Total FPL points scored
    pub total_points: i32,
    /// Whether player is in this gameweek's dream team
    pub in_dreamteam: bool,
}

/// Explains how a player's points were calculated for specific fixtures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPointsExplanation {
    /// Fixture ID where these points were earned
    pub fixture: i32,
    /// Breakdown of individual statistics and their point values
    pub stats: Vec<StatPointsBreakdown>,
}

/// Detailed breakdown of points awarded for a specific statistic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatPointsBreakdown {
    /// Name of the statistic (e.g., "goals_scored", "assists")
    pub identifier: String,
    /// Points awarded for this statistic
    pub points: i32,
    /// Number of times this statistic occurred
    pub value: i32,
    /// Any modification to the base points (rare)
    pub points_modification: i32,
}
