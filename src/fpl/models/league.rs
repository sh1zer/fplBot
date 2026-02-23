use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::from_value;

use crate::fpl::fpl_client;

/// Represents the complete standings data for a classic FPL league.
#[derive(Debug, Deserialize, Clone)]
pub struct LeagueStandings {
    #[serde(rename = "new_entries")]
    pub new_managers: NewManagers,

    #[serde(rename = "last_updated_data")]
    pub last_updated: DateTime<Utc>,

    #[serde(rename = "league")]
    pub league_info: LeagueInfo,

    pub standings: Standings,
}

/// Represents new manager entries in a league.
#[derive(Debug, Deserialize, Clone)]
pub struct NewManagers {
    pub has_next: bool,

    pub page: i32,

    #[serde(rename = "results")]
    pub managers: Vec<LeagueManager>,
}

/// Contains metadata about an FPL league.
#[derive(Debug, Deserialize, Clone)]
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

/// Represents the standings section of league data.
#[derive(Debug, Deserialize, Clone)]
pub struct Standings {
    pub has_next: bool,

    pub page: i32,

    #[serde(rename = "results")]
    pub managers: Vec<StandingsManager>,
}

/// Represents a manager's entry in a league (used for new entries).
#[derive(Debug, Deserialize, Clone)]
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

/// Represents a manager's position in league standings.
#[derive(Debug, Deserialize, Clone)]
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
    /// Fetches the first page of league standings.
    ///
    /// A convenience method that calls `fetch_page` with no page parameter.
    ///
    /// # Parameters
    ///
    /// * `league_id` - The ID of the classic league
    ///
    /// # Returns
    ///
    /// * `Ok(LeagueStandings)` - Successfully parsed league standings
    /// * `Err` - Network error, API error, or JSON parsing error
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use fplbot::fpl::models::league::LeagueStandings;
    ///
    /// let standings = LeagueStandings::fetch(314).await?;
    /// println!("League: {}", standings.league_info.league_name);
    /// ```
    pub async fn fetch(league_id: i32) -> Result<Self> {
        Self::fetch_page(league_id, 1).await
    }

    /// Fetches a specific page of league standings.
    ///
    /// # Parameters
    ///
    /// * `league_id` - The ID of the classic league
    /// * `page` - Optional page number (1-based). If `None`, fetches first page.
    ///
    /// # Returns
    ///
    /// * `Ok(LeagueStandings)` - Successfully parsed league standings
    /// * `Err` - Network error, API error, or JSON parsing error
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use fplbot::fpl::models::league::LeagueStandings;
    ///
    /// // Get first page
    /// let page1 = LeagueStandings::fetch_page(314, None).await?;
    ///
    /// // Get second page  
    /// let page2 = LeagueStandings::fetch_page(314, Some(2)).await?;
    /// ```
    pub async fn fetch_page(league_id: i32, page: i32) -> Result<Self> {
        let response = fpl_client().get_league_standings(league_id, page).await?;
        Ok(from_value(response)?)
    }
}
