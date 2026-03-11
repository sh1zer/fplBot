use crate::database::service::Database;
use anyhow::Result;

impl Database {
    pub async fn add_fixture_tracking(&self, channel_id: i64, fixture_id: i64) -> Result<()> {
        sqlx::query!(
            "INSERT OR IGNORE INTO channel_tracked_fixtures (channel_id, fixture_id) VALUES (?, ?)",
            channel_id,
            fixture_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_channels_tracking_fixture(&self, fixture_id: i64) -> Result<Vec<i64>> {
        let rows: Vec<i64> = sqlx::query!(
            "SELECT channel_id FROM channel_tracked_fixtures WHERE fixture_id=?",
            fixture_id
        )
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| r.channel_id)
        .collect();
        Ok(rows)
    }

    pub async fn get_fixtures_tracked_by_channel(&self, channel_id: i64) -> Result<Vec<i64>> {
        let rows: Vec<i64> = sqlx::query!(
            "SELECT fixture_id FROM channel_tracked_fixtures WHERE channel_id=?",
            channel_id
        )
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| r.fixture_id)
        .collect();
        Ok(rows)
    }

    pub async fn remove_fixture_tracking(&self, channel_id: i64, fixture_id: i64) -> Result<()> {
        sqlx::query!(
            "DELETE FROM channel_tracked_fixtures WHERE channel_id=? AND fixture_id=?",
            channel_id,
            fixture_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_player_tracking(&self, channel_id: i64, player_id: i64) -> Result<()> {
        sqlx::query!(
            "INSERT OR IGNORE INTO channel_tracked_players (channel_id, player_id) VALUES (?, ?)",
            channel_id,
            player_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_channels_tracking_player(&self, player_id: i64) -> Result<Vec<i64>> {
        let rows: Vec<i64> = sqlx::query!(
            "SELECT channel_id FROM channel_tracked_players WHERE player_id=?",
            player_id
        )
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| r.channel_id)
        .collect();
        Ok(rows)
    }

    pub async fn get_players_tracked_by_channel(&self, channel_id: i64) -> Result<Vec<i64>> {
        let rows: Vec<i64> = sqlx::query!(
            "SELECT player_id FROM channel_tracked_players WHERE channel_id=?",
            channel_id
        )
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| r.player_id)
        .collect();
        Ok(rows)
    }

    pub async fn remove_player_tracking(&self, channel_id: i64, player_id: i64) -> Result<()> {
        sqlx::query!(
            "DELETE FROM channel_tracked_players WHERE channel_id=? AND player_id=?",
            channel_id,
            player_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
