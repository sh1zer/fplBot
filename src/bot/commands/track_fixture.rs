use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ButtonStyle};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateButton};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use crate::fpl::models::league::{LeagueStandings, StandingsManager};
use anyhow::{Result, anyhow};
use tracing::{info, error};

pub fn register() -> CreateCommand {
    CreateCommand::new("track_fixture")
        .description("Get updates on given fixture")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "fixture_id",
                "The fixture ID"
            ).required(true)
        )
}
pub async fn test(){
    info!("Test function called in track_fixture");
}
