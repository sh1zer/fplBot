#![allow(unused_imports)]
use anyhow::{anyhow, Result};
use log::{error, info};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

use crate::database::models::DBChannel;
use crate::database::{models::DBUser, service::db_service};

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

pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let channel_id: ChannelId = command.channel_id;

    let league_id = extract_league_id(&command.data.options()[0])?;
    // should probably be fine to go [0] since the argument is required

    info!(
        "Attempting to update league_id to {} for channel {}",
        league_id, channel_id
    );

    let res = db_service()
        .update_channel(&DBChannel {
            channel_id: channel_id.into(),
            default_league_id: Some(league_id),
        })
        .await;

    match res {
        Ok(_) => Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("channel league_id update succesful"),
        )),
        Err(e) => {
            error!(
                "Failed to update league_id for channel {}: {}",
                channel_id, e
            );
            Err(anyhow!("Failed to update users manager_id"))
        }
    }
}

fn extract_league_id(option: &ResolvedOption) -> Result<i32> {
    let val = &option.value;
    match val {
        ResolvedValue::Integer(id) => Ok(*id as i32),
        _ => {
            error!("No valid league_id provided in command options");
            Err(anyhow!("Please provide a valid league ID"))
        }
    }
}
