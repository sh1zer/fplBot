use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manager {
    pub id: u32,
    pub team_name: String,
    pub first_name: String,
    pub last_name: String,
    pub leagues: Option<Vec<u32>>,
    pub total_points: u32,
    pub gw_points: u32,
}

impl Manager {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            team_name: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            leagues: None,
            total_points: 0,
            gw_points: 0,
        }
    }

    // Getters
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn team_name(&self) -> &str {
        &self.team_name
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn leagues(&self) -> Option<&Vec<u32>> {
        self.leagues.as_ref()
    }

    pub fn total_points(&self) -> u32 {
        self.total_points
    }

    pub fn gw_points(&self) -> u32 {
        self.gw_points
    }

    // Setters
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_team_name(&mut self, team_name: String) {
        self.team_name = team_name;
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn set_leagues(&mut self, leagues: Option<Vec<u32>>) {
        self.leagues = leagues;
    }

    pub fn set_total_points(&mut self, total_points: u32) {
        self.total_points = total_points;
    }

    pub fn set_gw_points(&mut self, gw_points: u32) {
        self.gw_points = gw_points;
    }
}