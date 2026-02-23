use anyhow::Result;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;

use crate::database::models::DBChannel;
use crate::database::service::db_service;

pub fn register() -> CreateCommand {
    CreateCommand::new("check_channel_league_id")
        .description("Check your channels default league id")
}

pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let channel_id = command.channel_id;
    let user: DBChannel = db_service().get_channel(channel_id).await?;

    match user.default_league_id {
        Some(id) => Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(format!("Your current default league id is: {}", id)),
        )),
        Option::None => Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("You do not currently have a set manager id"),
        )),
    }
}
