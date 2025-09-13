use serde::de::Error;
use serde::Deserializer;
use serde::{Deserialize, Serialize};

use crate::fpl::fpl_client;

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

    pub async fn refresh_data(&self) {
        let response = fpl_client().get_manager(self.id).await;
    }

    pub fn get_league_ids(&self) -> impl Iterator<Item = (i32, &str)> + '_ {
        self.leagues.iter().map(|league| (league.id, league.name.as_str()))
    }
}

#[derive(Deserialize, Debug)]
struct LeagueInfo{
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
