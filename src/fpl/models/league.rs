use serde::{Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use serde_json::from_value;

use crate::fpl::fpl_client;

/// Represents the complete standings data for a classic FPL league.
#[derive(Debug, Deserialize, Clone)]
pub struct LeagueStandings {
    /// Information about new managers joining the league
    #[serde(rename = "new_entries")]
    pub new_managers: NewManagers,
    /// Timestamp when the data was last updated
    #[serde(rename = "last_updated_data")]
    pub last_updated: DateTime<Utc>,
    /// General information about the league
    #[serde(rename = "league")]
    pub league_info: LeagueInfo,
    /// Current league standings
    pub standings: Standings,
}

/// Represents new manager entries in a league.
#[derive(Debug, Deserialize, Clone)]
pub struct NewManagers {
    /// Whether there are more new managers on the next page
    pub has_next: bool,
    /// Current page number
    pub page: i32,
    /// List of new managers
    #[serde(rename = "results")]
    pub managers: Vec<LeagueManager>,
}

/// Contains metadata about an FPL league.
#[derive(Debug, Deserialize, Clone)]
pub struct LeagueInfo {
    /// Unique league ID
    pub id: i32,
    /// League name
    #[serde(rename = "name")]
    pub league_name: String,
    /// When the league was created
    #[serde(rename = "created")]
    pub created_date: DateTime<Utc>,
    /// Whether the league is closed to new entries
    #[serde(rename = "closed")]
    pub is_closed: bool,
    /// Maximum number of managers allowed (if set)
    #[serde(rename = "max_entries")]
    pub max_managers: Option<i32>,
    /// Type of league (e.g., "s" for standard)
    #[serde(rename = "league_type")]
    pub scoring_type: String,
    /// Scoring method (e.g., "c" for classic)
    #[serde(rename = "scoring")]
    pub scoring_method: String,
    /// Manager ID of the league admin (if any)
    #[serde(rename = "admin_entry")]
    pub admin_manager_id: Option<i32>,
    /// Gameweek when league scoring started
    #[serde(rename = "start_event")]
    pub start_gameweek: i32,
    /// Privacy setting (e.g., "p" for public)
    #[serde(rename = "code_privacy")]
    pub privacy_setting: String,
    /// Whether the league has a cup competition
    #[serde(rename = "has_cup")]
    pub has_cup_competition: bool,
    /// ID of associated cup league (if any)
    #[serde(rename = "cup_league")]
    pub cup_league_id: Option<i32>,
    /// Overall rank of this league (if applicable)
    #[serde(rename = "rank")]
    pub league_rank: Option<i32>,
}

/// Represents the standings section of league data.
#[derive(Debug, Deserialize, Clone)]
pub struct Standings {
    /// Whether there are more standings on the next page
    pub has_next: bool,
    /// Current page number
    pub page: i32,
    /// List of managers in the standings
    #[serde(rename = "results")]
    pub managers: Vec<StandingsManager>,
}

/// Represents a manager's entry in a league (used for new entries).
#[derive(Debug, Deserialize, Clone)]
pub struct LeagueManager {
    /// Manager's entry ID
    pub id: i32,
    /// League name
    #[serde(rename = "name")]
    pub league_name: String,
    /// When the manager joined
    #[serde(rename = "created")]
    pub created_date: DateTime<Utc>,
    /// Whether the league is closed
    #[serde(rename = "closed")]
    pub is_closed: bool,
    /// League admin's manager ID
    #[serde(rename = "admin_entry")]
    pub admin_manager_id: i32,
    /// Gameweek when league started
    #[serde(rename = "start_event")]
    pub start_gameweek: i32,
    /// Whether this manager can leave the league
    #[serde(rename = "entry_can_leave")]
    pub can_leave: bool,
    /// Whether this manager can admin the league
    #[serde(rename = "entry_can_admin")]
    pub can_admin: bool,
    /// Whether this manager can invite others
    #[serde(rename = "entry_can_invite")]
    pub can_invite: bool,
    /// Total number of managers in the league
    #[serde(rename = "rank_count")]
    pub total_managers: i32,
    /// Manager's percentile rank
    #[serde(rename = "entry_percentile_rank")]
    pub percentile_rank: i32,
    /// Manager's current rank
    #[serde(rename = "entry_rank")]
    pub current_rank: i32,
    /// Manager's previous rank
    #[serde(rename = "entry_last_rank")]
    pub previous_rank: i32,
}

/// Represents a manager's position in league standings.
#[derive(Debug, Deserialize, Clone)]
pub struct StandingsManager {
    /// Manager's unique ID
    pub id: i32,
    /// Points scored in the current gameweek
    #[serde(rename = "event_total")]
    pub gameweek_points: i32,
    /// Manager's display name
    #[serde(rename = "player_name")]
    pub manager_name: String,
    /// Current position in the league
    #[serde(rename = "rank")]
    pub current_rank: i32,
    /// Previous position in the league
    #[serde(rename = "last_rank")]
    pub previous_rank: i32,
    /// Rank used for sorting
    #[serde(rename = "rank_sort")]
    pub sort_rank: i32,
    /// Total points accumulated
    #[serde(rename = "total")]
    pub total_points: i32,
    /// Manager's entry/team ID
    #[serde(rename = "entry")]
    pub manager_id: i32,
    /// Manager's team name
    #[serde(rename = "entry_name")]
    pub team_name: String,
    /// Whether the manager has played this gameweek
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
    /// ```
    /// use fplbot::fpl::models::league::LeagueStandings;
    /// 
    /// let standings = LeagueStandings::fetch(314).await?;
    /// println!("League: {}", standings.league_info.league_name);
    /// ```
    pub async fn fetch(league_id: i32) -> Result<Self>{
        Self::fetch_page(league_id, None).await
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
    /// ```
    /// use fplbot::fpl::models::league::LeagueStandings;
    /// 
    /// // Get first page
    /// let page1 = LeagueStandings::fetch_page(314, None).await?;
    /// 
    /// // Get second page  
    /// let page2 = LeagueStandings::fetch_page(314, Some(2)).await?;
    /// ```
    pub async fn fetch_page(league_id: i32, page: Option<i32>) -> Result<Self>{
        let response = fpl_client().get_league_standings(league_id, page).await?;
        Ok(from_value(response)?)
    }
}
