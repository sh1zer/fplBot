//! Manager-related Discord commands
//!
//! Provides functionality for users to manage their FPL manager association
//! within the Discord bot, including setting and updating manager IDs.

use anyhow::{anyhow, Ok, Result};
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    ResolvedOption, ResolvedValue,
};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;
use tracing::{error, info};

use crate::database::{models::DBUser, service::db_service};

/// Registers the update_manager_id command with Discord
///
/// Creates the command definition for the `/update_manager_id` slash command
/// that allows users to set their FPL manager ID.
///
/// # Returns
/// * `CreateCommand` - Discord command definition ready for registration
pub fn register() -> CreateCommand {
    CreateCommand::new("update_channel_league_id")
        .description("Update the channels defualt league id")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "league_id",
                "Your FPL league id",
            )
            .required(true),
        )
}

// pub async fn run(
//     _ctx: &Context,
//     command: &CommandInteraction,
// ) -> Result<CreateInteractionResponse> {
//     let channel_id = command.channel_id;
//
//     let league_id = command.data.options().first();
//
//     info!(
//         "Attempting to update league_id to {} for channel {}",
//         league_id as i32, channel_id
//     );
//
//     Ok(().into())
// }
