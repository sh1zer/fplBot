use anyhow::anyhow;
use anyhow::Result;
use serenity::all::{ChannelId, UserId};
use sqlx::SqlitePool;
use std::option::Option;

use crate::database::models::{DBChannel, DBUser};

use tokio::sync::OnceCell;

static DB_SERVICE: OnceCell<Database> = OnceCell::const_new();

pub async fn init_db_service() -> Result<()> {
    let database = Database::setup().await?;
    DB_SERVICE
        .set(database)
        .map_err(|_| anyhow::anyhow!("Database service already initialized"))?;
    Ok(())
}

pub fn db_service() -> &'static Database {
    DB_SERVICE
        .get()
        .expect("Database service not initialized - call init_db_service() first")
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    async fn setup() -> Result<Self> {
        let pool = SqlitePool::connect("sqlite:fplbot.db").await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn get_user(&self, discord_id: UserId) -> Result<DBUser> {
        let id = i64::from(discord_id);
        let row = sqlx::query!(
            "SELECT discord_id, manager_id FROM users WHERE discord_id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        let user = match row {
            Option::Some(r) => DBUser {
                discord_id: r.discord_id,
                manager_id: r.manager_id.map(|i| i as i32),
            },
            Option::None => return Err(anyhow!("User {} not found", discord_id)),
        };
        Ok(user)
    }

    pub async fn update_user(&self, user: &DBUser) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO users (discord_id, manager_id) VALUES (?, ?)",
            user.discord_id,
            user.manager_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_channel(&self, channel_id: ChannelId) -> Result<DBChannel> {
        let id = i64::from(channel_id);
        let row = sqlx::query!(
            "SELECT channel_id, default_league_id FROM channels WHERE channel_id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Option::Some(r) => Ok(DBChannel {
                channel_id: r.channel_id,
                default_league_id: r.default_league_id.map(|i| i as i32),
            }),
            Option::None => Err(anyhow!("User not found")),
        }
    }

    pub async fn update_channel(&self, channel: &DBChannel) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO channels (channel_id, default_league_id) VALUES (?, ?)",
            channel.channel_id,
            channel.default_league_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
