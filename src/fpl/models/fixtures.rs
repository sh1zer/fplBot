use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::from_value;

use crate::fpl::fpl_client;

// for endpoint fixtures/?event={event_id}

/// Represents a collection of fixtures for a specific gameweek.
#[derive(Debug, Clone, Deserialize)]
pub struct GameweekFixtures {
    pub gameweek: i32,

    pub fixtures: Vec<Fixture>,
}

/// Represents a single Premier League fixture.
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub id: u32,

    pub code: u64,

    pub event: Option<u8>,

    pub team_h: i32,

    pub team_a: i32,

    pub team_h_score: Option<u8>,

    pub team_a_score: Option<u8>,

    pub finished: bool,

    pub finished_provisional: bool,

    pub started: bool,

    pub kickoff_time: DateTime<Utc>,

    pub minutes: u16,

    pub team_h_difficulty: u8,

    pub team_a_difficulty: u8,

    pub stats: Vec<FixtureStat>,

    pub provisional_start_time: bool,

    pub pulse_id: u64,
}

/// Represents a statistic for a fixture (e.g., goals, assists).
#[derive(Debug, Clone, Deserialize)]
pub struct FixtureStat {
    pub identifier: String,

    pub a: Vec<StatEntry>,

    pub h: Vec<StatEntry>,
}

/// Represents a single statistical entry for a player in a fixture.
#[derive(Debug, Clone, Deserialize)]
pub struct StatEntry {
    pub value: i32,

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
/// ```ignore
/// use fplbot::fpl::models::fixtures::fetch_fixtures;
///
/// let fixtures = fetch_fixtures(10).await?;
/// println!("Found {} fixtures for gameweek {}", fixtures.fixtures.len(), fixtures.gameweek);
/// ```
pub async fn fetch_fixtures(gameweek: i32) -> Result<GameweekFixtures> {
    let response = fpl_client().get_fixtures(Some(gameweek)).await?;
    Ok(GameweekFixtures {
        gameweek,
        fixtures: from_value(response)?,
    })
}
