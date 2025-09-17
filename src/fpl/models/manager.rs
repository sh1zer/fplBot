use serde::de::Error;
use serde::Deserializer;
use serde::{Deserialize, Serialize};

use crate::fpl::fpl_client;

/// Represents an FPL manager with their team and performance data.
#[derive(Debug, Deserialize)]
pub struct Manager {
    /// Unique manager ID
    pub id: i32,
    /// Team name chosen by the manager
    #[serde(rename = "name")]
    pub team_name: String,
    /// Manager's first name
    #[serde(rename = "player_first_name")]
    pub first_name: String,
    /// Manager's last name
    #[serde(rename = "player_last_name")]
    pub last_name: String,
    /// List of classic leagues the manager is in
    #[serde(deserialize_with = "deserialize_classic_leagues")]
    leagues: Vec<LeagueInfo>,
    /// Total points accumulated across all gameweeks
    #[serde(rename = "summary_overall_points")]
    pub total_points: i32,
    /// Points scored in the current gameweek
    #[serde(rename = "summary_event_points")]
    pub gw_points: i32,
}

impl Manager {
    /// Creates a new Manager instance with the given ID.
    /// 
    /// All other fields are initialized to default values and should be
    /// populated by calling `refresh_data()`.
    /// 
    /// # Parameters
    /// 
    /// * `id` - The FPL manager ID
    /// 
    /// # Returns
    /// 
    /// A new `Manager` instance with default field values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use fplbot::fpl::models::manager::Manager;
    /// 
    /// let manager = Manager::new(123456);
    /// assert_eq!(manager.id, 123456);
    /// ```
    pub fn new(id: i32) -> Self {
        Self {
            id,
            team_name: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            leagues: vec![],
            total_points: 0,
            gw_points: 0,
        }
    }

    /// Refreshes the manager's data from the FPL API.
    /// 
    /// This method fetches the latest information about the manager
    /// from the FPL API but currently does not update the struct fields.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let manager = Manager::new(123456);
    /// manager.refresh_data().await;
    /// ```
    pub async fn refresh_data(&self) {
        let response = fpl_client().get_manager_summary(self.id).await;
    }

    /// Returns an iterator over the manager's league IDs and names.
    /// 
    /// # Returns
    /// 
    /// An iterator yielding tuples of (league_id, league_name).
    /// 
    /// # Examples
    /// 
    /// ```
    /// for (league_id, league_name) in manager.get_league_ids() {
    ///     println!("League {}: {}", league_id, league_name);
    /// }
    /// ```
    pub fn get_league_ids(&self) -> impl Iterator<Item = (i32, &str)> + '_ {
        self.leagues.iter().map(|league| (league.id, league.name.as_str()))
    }

}

/// Contains information about a league that a manager is participating in.
#[derive(Deserialize, Debug)]
struct LeagueInfo{
    /// League ID
    id: i32,
    /// League name
    name: String,
    /// When the manager joined (ISO string)
    created: String,
    /// Whether the league is closed to new entries
    closed: bool,
    /// Manager ID of the league admin
    admin_entry: i32,
    /// Gameweek when the league started scoring
    start_event: i32,
    /// Whether this manager can leave the league
    entry_can_leave: bool,
    /// Whether this manager can admin the league
    entry_can_admin: bool,
    /// Whether this manager can invite others
    entry_can_invite: bool,
    /// Total number of managers in the league
    rank_count: i32,
    /// Manager's percentile rank in the league
    entry_percentile_rank: i32,
    /// Manager's current rank in the league
    entry_rank: i32,
    /// Manager's previous rank in the league
    entry_last_rank: i32,
} 

/// Custom deserializer for classic leagues data.
/// 
/// The FPL API returns leagues in a nested structure with separate arrays
/// for classic and head-to-head leagues. This function extracts only the
/// classic leagues.
fn deserialize_classic_leagues<'de, D>(deserializer: D) -> Result<Vec<LeagueInfo>, D::Error>
where D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct LeaguesWrapper{
        classic: Vec<LeagueInfo>,
    }
    let wrapper = LeaguesWrapper::deserialize(deserializer)?;
    Ok(wrapper.classic)
}
