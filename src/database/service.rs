use anyhow::anyhow;
use sqlx::SqlitePool;
use anyhow::Result;

use crate::database::models::{DBChannel, DBUser};

use tokio::sync::OnceCell;

static DB_SERVICE: OnceCell<Database> = OnceCell::const_new();

pub async fn init_db_service() -> Result<()> {
    let database = Database::setup().await?;
    DB_SERVICE.set(database)
        .map_err(|_| anyhow::anyhow!("Database service already initialized"))?;
    Ok(())
}

pub fn db_service() -> &'static Database {
    DB_SERVICE.get()
        .expect("Database service not initialized - call init_db_service() first")
}



pub struct Database {
    pool: SqlitePool,
}

impl Database {
    async fn setup() -> Result<Self>{
        let pool = SqlitePool::connect("sqlite:fplbot.db").await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self{pool})
    }

    pub async fn get_user(&self, discord_id: &str) -> Result<DBUser> {
        let row = sqlx::query!(
            "SELECT discord_id, manager_id FROM users WHERE discord_id = ?",
            discord_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let user = match row{
            Some(r) => DBUser { 
                discord_id: r.discord_id.ok_or(anyhow!("User not found"))?,
                manager_id: r.manager_id.map(|i| i as i32),
            },
            None => return Err(anyhow!("User not found"))
        };
        Ok(user)
    }

    pub async fn update_user(&self, user: &DBUser) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO users (discord_id, manager_id) VALUES (?, ?)",
            user.discord_id, user.manager_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
    pub async fn get_channel(&self, channel_id: &str) -> Result<DBChannel> {
        let row = sqlx::query!(
            "SELECT channel_id, default_league_id FROM channels WHERE channel_id = ?",
            channel_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let user = match row{
            Some(r) => DBChannel{
                channel_id: r.channel_id.ok_or(anyhow!("User not found"))?,
                default_league_id: r.default_league_id.map(|i| i as i32),
            },
            None => return Err(anyhow!("User not found"))
        };
        Ok(user)
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
