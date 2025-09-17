
/// Represents a Premier League team with full name and short name.
#[derive(Debug, Clone)]
pub struct TeamName {
    /// Full team name (e.g., "Arsenal")
    pub name: &'static str,
    /// Short team name/abbreviation (e.g., "ARS")
    pub short_name: &'static str,
}

/// Returns team name information for a given FPL team ID.
/// 
/// Maps the FPL API team IDs to their corresponding team names and abbreviations.
/// This is useful for displaying human-readable team names in Discord messages.
/// 
/// # Parameters
/// 
/// * `id` - The FPL team ID (1-20 for Premier League teams)
/// 
/// # Returns
/// 
/// A `TeamName` struct containing the full name and short name. Returns 
/// "Unknown" team if the ID is not recognized.
/// 
/// # Examples
/// 
/// ```
/// use fplbot::fpl::models::teams::get_team_name;
/// 
/// let arsenal = get_team_name(1);
/// assert_eq!(arsenal.name, "Arsenal");
/// assert_eq!(arsenal.short_name, "ARS");
/// ```
pub fn get_team_name(id: i32) -> TeamName {
    match id {
        1 => TeamName { name: "Arsenal", short_name: "ARS" },
        2 => TeamName { name: "Aston Villa", short_name: "AVL" },
        3 => TeamName { name: "Burnley", short_name: "BUR" },
        4 => TeamName { name: "Bournemouth", short_name: "BOU" },
        5 => TeamName { name: "Brentford", short_name: "BRE" },
        6 => TeamName { name: "Brighton", short_name: "BHA" },
        7 => TeamName { name: "Chelsea", short_name: "CHE" },
        8 => TeamName { name: "Crystal Palace", short_name: "CRY" },
        9 => TeamName { name: "Everton", short_name: "EVE" },
        10 => TeamName { name: "Fulham", short_name: "FUL" },
        11 => TeamName { name: "Leeds", short_name: "LEE" },
        12 => TeamName { name: "Liverpool", short_name: "LIV" },
        13 => TeamName { name: "Man City", short_name: "MCI" },
        14 => TeamName { name: "Man Utd", short_name: "MUN" },
        15 => TeamName { name: "Newcastle", short_name: "NEW" },
        16 => TeamName { name: "Nott'm Forest", short_name: "NFO" },
        17 => TeamName { name: "Sunderland", short_name: "SUN" },
        18 => TeamName { name: "Spurs", short_name: "TOT" },
        19 => TeamName { name: "West Ham", short_name: "WHU" },
        20 => TeamName { name: "Wolves", short_name: "WOL" },
        _ => TeamName { name: "Unknown", short_name: "???" },
    }
}
