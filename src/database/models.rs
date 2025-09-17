
pub struct DBUser {
    pub discord_id: String,
    pub manager_id: Option<i32>,
}

pub struct DBChannel {
    pub channel_id: String,
    pub default_league_id: Option<i32>,
}
