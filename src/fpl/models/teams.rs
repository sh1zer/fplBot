
#[derive(Debug, Clone)]
pub struct TeamName {
    pub name: &'static str,
    pub short_name: &'static str,
}

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
