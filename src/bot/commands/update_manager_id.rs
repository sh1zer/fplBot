//! Manager-related Discord commands
//!
//! Provides functionality for users to manage their FPL manager association
//! within the Discord bot, including setting and updating manager IDs.

use anyhow::{anyhow, Result};
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
    CreateCommand::new("update_manager_id")
        .description("Update your own manager id")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "manager_id",
                "Your FPL manager id",
            )
            .required(true),
        )
}

/// Main handler for the `/update_manager_id` slash command
///
/// Allows Discord users to set or update their FPL manager ID in the database.
/// This association enables the bot to provide personalized FPL data and use
/// default manager IDs in commands.
///
/// # Arguments
/// * `_ctx` - Discord context (unused in current implementation)
/// * `command` - The slash command interaction containing user input
///
/// # Returns
/// * `Result<CreateInteractionResponse>` - Success or error response
///
/// # Errors
/// Returns error if:
/// - Manager ID is not provided or invalid
/// - Database update operation fails
/// - User cannot be identified
///
/// # Example Usage
/// `/update_manager_id manager_id:123456`
pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let user_id = command.user.id;
    let user_name = &command.user.name;

    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(id),
        ..
    }) = command.data.options().first()
    {
        info!(
            "Attempting to update manager_id to {} for user {} ({})",
            id, user_name, user_id
        );
        let res = db_service()
            .update_user(&DBUser {
                discord_id: i64::from(user_id),
                manager_id: Some(*id as i32),
            })
            .await;

        match res {
            Ok(_) => {
                info!(
                    "Successfully updated manager_id to {} for user {}",
                    id, user_name
                );
                Ok(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Manager_id update succesful"),
                ))
            }
            Err(e) => {
                error!("Failed to update manager_id for user {}: {}", user_name, e);
                Err(anyhow!("Failed to update users manager_id"))
            }
        }
    } else {
        error!(
            "No manager_id provided in command options for user {}",
            user_id
        );
        Err(anyhow!("Failed to find users discord_id"))
    }
}
