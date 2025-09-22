//! Fixture tracking Discord commands
//!
//! Provides functionality for users to track specific FPL fixtures and receive
//! updates about goals, cards, and other match events.
//!
//! # TODO
//! This module is currently under development and needs implementation for:
//! - Fixture tracking subscription system
//! - Real-time match event notifications
//! - User notification preferences
//! - Integration with database for persistent tracking

use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ButtonStyle};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateButton};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use crate::fpl::models::league::{LeagueStandings, StandingsManager};
use anyhow::{Result, anyhow};
use tracing::{info, error};

/// Registers the track_fixture command with Discord
///
/// Creates the command definition for the `/track_fixture` slash command that allows
/// users to subscribe to fixture updates.
///
/// # Returns
/// * `CreateCommand` - Discord command definition ready for registration
///
/// # TODO
/// Implement the actual command handler and tracking functionality
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

// pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
//
// }
