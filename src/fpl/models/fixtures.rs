use chrono::{DateTime, Utc};
use serde::{Deserialize};
use anyhow::Result;
use serde_json::from_value;

use crate::fpl::fpl_client;

// for endpoint fixtures/?event={event_id}

/// Represents a collection of fixtures for a specific gameweek.
#[derive(Debug, Clone, Deserialize)]
pub struct GameweekFixtures{
    /// The gameweek number these fixtures belong to
    pub gameweek: i32,
    /// List of all fixtures in this gameweek
    pub fixtures: Vec<Fixture>,
}

/// Represents a single Premier League fixture.
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    /// Unique fixture ID
    pub id: u32,
    /// Fixture code
    pub code: u64,
    /// Gameweek number (event)
    pub event: Option<u8>,
    /// Home team ID
    pub team_h: i32,
    /// Away team ID
    pub team_a: i32,
    /// Home team final score (if finished)
    pub team_h_score: Option<u8>,
    /// Away team final score (if finished)
    pub team_a_score: Option<u8>,
    /// Whether the fixture has finished
    pub finished: bool,
    /// Whether the fixture result is provisional
    pub finished_provisional: bool,
    /// Whether the fixture has started
    pub started: bool,
    /// Scheduled kickoff time
    pub kickoff_time: DateTime<Utc>,
    /// Minutes played in the fixture
    pub minutes: u16,
    /// FPL difficulty rating for home team (1-5)
    pub team_h_difficulty: u8,
    /// FPL difficulty rating for away team (1-5)
    pub team_a_difficulty: u8,
    /// Match statistics
    pub stats: Vec<FixtureStat>,
    /// Whether the start time is provisional
    pub provisional_start_time: bool,
    /// Pulse ID for the fixture
    pub pulse_id: u64,
}

/// Represents a statistic for a fixture (e.g., goals, assists).
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureStat {
    /// Type of statistic (e.g., "goals_scored", "assists")
    pub identifier: String,
    /// Away team statistics
    pub a: Vec<StatEntry>,
    /// Home team statistics
    pub h: Vec<StatEntry>,
}

/// Represents a single statistical entry for a player in a fixture.
#[derive(Debug, Clone, Deserialize)]
pub struct StatEntry {
    /// Statistical value (e.g., number of goals)
    pub value: i32,
    /// Player ID (element)
    pub element: i32,
}


/// Fetches fixture data for a specific gameweek.
/// 
/// Retrieves all fixtures scheduled for the given gameweek from the FPL API.
/// This includes fixture details, team information, scores (if finished), and
/// match statistics.
/// 
/// # Parameters
/// 
/// * `gameweek` - The gameweek number to fetch fixtures for
/// 
/// # Returns
/// 
/// * `Ok(GameweekFixtures)` - Successfully parsed fixture data
/// * `Err` - Network error, API error, or JSON parsing error
/// 
/// # Examples
/// 
/// ```
/// use fplbot::fpl::models::fixtures::fetch_fixtures;
/// 
/// let fixtures = fetch_fixtures(10).await?;
/// println!("Found {} fixtures for gameweek {}", fixtures.fixtures.len(), fixtures.gameweek);
/// ```
pub async fn fetch_fixtures(gameweek: i32) -> Result<GameweekFixtures>{
    let response = fpl_client().get_fixtures(Some(gameweek)).await?;
    Ok(
        GameweekFixtures{
            gameweek,
            fixtures: from_value(response)?
        })
}



//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LiveGameweekData {
//     pub elements: Vec<PlayerLiveStats>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PlayerLiveStats {
//     pub id: u32,
//     pub stats: LiveStats,
//     pub explain: Vec<PointsExplanation>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LiveStats {
//     pub minutes: u16,
//     pub goals_scored: u8,
//     pub assists: u8,
//     pub clean_sheets: u8,
//     pub goals_conceded: u8,
//     pub own_goals: u8,
//     pub penalties_saved: u8,
//     pub penalties_missed: u8,
//     pub yellow_cards: u8,
//     pub red_cards: u8,
//     pub saves: u16,
//     pub bonus: u8,
//     pub bps: u16,
//     pub influence: String,
//     pub creativity: String,
//     pub threat: String,
//     pub ict_index: String,
//     pub starts: u8,
//     pub expected_goals: String,
//     pub expected_assists: String,
//     pub expected_goal_involvements: String,
//     pub expected_goals_conceded: String,
//     pub total_points: i16,
//     pub in_dreamteam: bool,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PointsExplanation {
//     pub fixture: u32,
//     pub stats: Vec<StatPoint>,
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct StatPoint {
//     pub identifier: String,
//     pub points: i8,
//     pub value: u32,
// }
