use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption, ResolvedValue};
use anyhow::{Result, anyhow};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType};
use tracing::{info, error};

use crate::database::{models::DBUser, service::db_service};

pub fn register() -> CreateCommand {
    CreateCommand::new("update_manager_id")
        .description("Update your own manager id")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "manager_id",
                "Your FPL manager id"
            ).required(true)
        )
}

pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
    let user_id = &command.user.name;
    info!("Processing update_manager_id command for user {}", user_id);

    if let Some(ResolvedOption { value: ResolvedValue::Integer(id), ..}) = command.data.options().first(){
        info!("Attempting to update manager_id to {} for user {}", id, user_id);
        let res = db_service().update_user(&DBUser{ discord_id: user_id.to_string(), manager_id: Some(*id as i32) }).await;
        match res{
            Ok(_) => {
                info!("Successfully updated manager_id to {} for user {}", id, user_id);
                Ok(CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                    .content("Manager_id update succesful")
                ))
            }
            Err(e) => {
                error!("Failed to update manager_id for user {}: {}", user_id, e);
                Err(anyhow!("Failed to update users manager_id"))
            }
        }
    }
    else{
        error!("No manager_id provided in command options for user {}", user_id);
        Err(anyhow!("Failed to find users discord_id"))
    }
}

