use serde::Deserialize;
use serde::Deserializer;

use crate::fpl::fpl_client;

/// Represents an FPL manager with their team and performance data.
#[derive(Debug, Deserialize)]
pub struct Manager {
    pub id: i32,

    #[serde(rename = "name")]
    pub team_name: String,

    #[serde(rename = "player_first_name")]
    pub first_name: String,

    #[serde(rename = "player_last_name")]
    pub last_name: String,

    #[serde(deserialize_with = "deserialize_classic_leagues")]
    leagues: Vec<LeagueInfo>,

    #[serde(rename = "summary_overall_points")]
    pub total_points: i32,

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
        let _response = fpl_client().get_manager_summary(self.id).await;
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
        self.leagues
            .iter()
            .map(|league| (league.id, league.name.as_str()))
    }
}

/// Contains information about a league that a manager is participating in.
#[derive(Deserialize, Debug)]
struct LeagueInfo {
    id: i32,

    name: String,

    created: String,

    closed: bool,

    admin_entry: i32,

    start_event: i32,

    entry_can_leave: bool,

    entry_can_admin: bool,

    entry_can_invite: bool,

    rank_count: i32,

    entry_percentile_rank: i32,

    entry_rank: i32,

    entry_last_rank: i32,
}

/// Custom deserializer for classic leagues data.
///
/// The FPL API returns leagues in a nested structure with separate arrays
/// for classic and head-to-head leagues. This function extracts only the
/// classic leagues.
fn deserialize_classic_leagues<'de, D>(deserializer: D) -> Result<Vec<LeagueInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct LeaguesWrapper {
        classic: Vec<LeagueInfo>,
    }
    let wrapper = LeaguesWrapper::deserialize(deserializer)?;
    Ok(wrapper.classic)
}
