use serde::{Deserialize, Serialize};

/// Represents a Fantasy Premier League player with comprehensive statistics and metadata.
/// 
/// This struct contains all the data returned by the FPL API for a player, including
/// performance statistics, pricing information, injury status, and ranking data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Percentage chance of playing in the next round (0-100, None if not applicable)
    pub chance_of_playing_next_round: Option<i32>,
    /// Percentage chance of playing in the current round (0-100, None if not applicable)
    pub chance_of_playing_this_round: Option<i32>,
    /// Player's unique code
    pub code: i32,
    /// Price change in the current gameweek (in 0.1m increments)
    pub cost_change_event: i32,
    /// Number of price falls in the current gameweek
    pub cost_change_event_fall: i32,
    /// Total price change since the season start (in 0.1m increments)
    pub cost_change_start: i32,
    /// Number of price falls since the season start
    pub cost_change_start_fall: i32,
    /// Number of times the player has been in the gameweek dream team
    pub dreamteam_count: i32,
    /// Player position (1=GK, 2=DEF, 3=MID, 4=FWD)
    pub element_type: i32,
    /// Expected points for next gameweek (as string)
    pub ep_next: String,
    /// Expected points for current gameweek (as string)  
    pub ep_this: String,
    /// Points scored in the current gameweek
    pub event_points: i32,
    /// Player's first name
    pub first_name: String,
    /// Player's recent form rating (as string)
    pub form: String,
    /// Player's unique ID
    pub id: i32,
    /// Whether the player is in the current gameweek's dream team
    pub in_dreamteam: bool,
    /// Latest news about the player (injuries, suspensions, etc.)
    pub news: String,
    /// When the news was added (ISO string, None if no news)
    pub news_added: Option<String>,
    /// Current price in 0.1m increments (e.g., 100 = £10.0m)
    pub now_cost: i32,
    /// Filename of player's photo
    pub photo: String,
    /// Average points per game (as string)
    pub points_per_game: String,
    /// Player's surname
    pub second_name: String,
    /// Percentage of managers who own this player (as string)
    pub selected_by_percent: String,
    /// Whether the player has special status
    pub special: bool,
    /// Player's squad number (None if not assigned)
    pub squad_number: Option<i32>,
    /// Player's availability status (a=available, i=injured, s=suspended, etc.)
    pub status: String,
    /// Team ID the player belongs to
    pub team: i32,
    /// Team code
    pub team_code: i32,
    /// Total points accumulated this season
    pub total_points: i32,
    /// Total transfers in this season
    pub transfers_in: i32,
    /// Transfers in during current gameweek
    pub transfers_in_event: i32,
    /// Total transfers out this season
    pub transfers_out: i32,
    /// Transfers out during current gameweek
    pub transfers_out_event: i32,
    /// Value for money based on recent form (as string)
    pub value_form: String,
    /// Value for money based on season performance (as string)
    pub value_season: String,
    /// Short display name used on the website
    pub web_name: String,
    /// Total minutes played this season
    pub minutes: i32,
    /// Total goals scored this season
    pub goals_scored: i32,
    /// Total assists this season
    pub assists: i32,
    /// Total clean sheets this season (defenders/goalkeepers)
    pub clean_sheets: i32,
    /// Total goals conceded this season (defenders/goalkeepers)
    pub goals_conceded: i32,
    /// Total own goals this season
    pub own_goals: i32,
    /// Total penalties saved this season (goalkeepers)
    pub penalties_saved: i32,
    /// Total penalties missed this season
    pub penalties_missed: i32,
    /// Total yellow cards this season
    pub yellow_cards: i32,
    /// Total red cards this season
    pub red_cards: i32,
    /// Total saves this season (goalkeepers)
    pub saves: i32,
    /// Total bonus points this season
    pub bonus: i32,
    /// Total Bonus Points System score this season
    pub bps: i32,
    /// ICT Influence score (as string)
    pub influence: String,
    /// ICT Creativity score (as string)
    pub creativity: String,
    /// ICT Threat score (as string)
    pub threat: String,
    /// Combined ICT Index score (as string)
    pub ict_index: String,
    /// Total games started this season
    pub starts: i32,
    /// Expected goals this season (as string)
    pub expected_goals: String,
    /// Expected assists this season (as string)
    pub expected_assists: String,
    /// Expected goal involvements this season (as string)
    pub expected_goal_involvements: String,
    /// Expected goals conceded this season (as string)
    pub expected_goals_conceded: String,
    /// Rank for Influence among all players
    pub influence_rank: i32,
    /// Rank for Influence among players in the same position
    pub influence_rank_type: i32,
    /// Rank for Creativity among all players
    pub creativity_rank: i32,
    /// Rank for Creativity among players in the same position
    pub creativity_rank_type: i32,
    /// Rank for Threat among all players
    pub threat_rank: i32,
    /// Rank for Threat among players in the same position
    pub threat_rank_type: i32,
    /// Rank for ICT Index among all players
    pub ict_index_rank: i32,
    /// Rank for ICT Index among players in the same position
    pub ict_index_rank_type: i32,
    /// Order preference for corners and indirect free kicks (None if not applicable)
    pub corners_and_indirect_freekicks_order: Option<i32>,
    /// Description of corner/indirect free kick taking status
    pub corners_and_indirect_freekicks_text: String,
    /// Order preference for direct free kicks (None if not applicable)
    pub direct_freekicks_order: Option<i32>,
    /// Description of direct free kick taking status
    pub direct_freekicks_text: String,
    /// Order preference for penalties (None if not applicable)
    pub penalties_order: Option<i32>,
    /// Description of penalty taking status
    pub penalties_text: String,
}