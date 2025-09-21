
pub struct DBUser {
    pub discord_id: i64,
    pub manager_id: Option<i32>,
}

pub struct DBChannel {
    pub channel_id: i64,
    pub default_league_id: Option<i32>,
}
