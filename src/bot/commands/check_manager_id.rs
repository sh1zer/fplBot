use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage};
use anyhow::{Result};
use serenity::builder::{CreateCommand};

use crate::database::{service::db_service};

pub fn register() -> CreateCommand {
    CreateCommand::new("check_manager_id")
        .description("Check your own manager id")
}

pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
    let user_id = command.user.id;
    let user = db_service().get_user(user_id).await?;
    
    match user.manager_id{
        Some(id) => Ok(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!("Your current manager id is: {}", id)))),
        None => Ok(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("You do not currently have a set manager id"))),
    }
}

