#![allow(dead_code)]

use dotenvy::dotenv;
use crate::database::service::init_db_service;
use serenity::prelude::*;
use std::env;
use log::{error, info};

mod bot;
mod utils;
mod database;

use bot::handlers::Handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let token = env::var("BOT_TOKEN")
        .expect("Expected BOT_TOKEN in environment");

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    init_db_service().await?;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    info!("Starting bot...");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
