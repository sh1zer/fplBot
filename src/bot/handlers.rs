use serenity::{
    all::{Command, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready, ComponentInteraction}, async_trait, prelude::*
};
use tracing::info;

use crate::{bot::commands, fpl};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);

        let commands = vec![
            CreateCommand::new("hello").description("Say hello to the bot"),
            commands::standings::register(),
        ];

        match Command::set_global_commands(&ctx.http, commands).await {
            Ok(_) => info!("Successfully registered slash commands"),
            Err(e) => info!("Failed to register slash commands: {}", e),
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let response = match command.data.name.as_str() {
                    "hello" => {
                        let data = CreateInteractionResponseMessage::new().content("Hey!");
                        Ok(CreateInteractionResponse::Message(data))
                    },
                    "standings" => {
                        commands::standings::run(&ctx, &command).await
                    },
                    _ => {
                        let data = CreateInteractionResponseMessage::new().content("Unknown command");
                        Ok(CreateInteractionResponse::Message(data))
                    },
                };

                if let Err(why) = match response {
                    Ok(response) => command.create_response(&ctx.http, response).await,
                    Err(e) => {
                        let error_response = CreateInteractionResponseMessage::new()
                            .content(format!("Error: {}", e));
                        command.create_response(&ctx.http, CreateInteractionResponse::Message(error_response)).await
                    }
                } {
                    info!("Cannot respond to slash command: {}", why);
                }
            },
            Interaction::Component(component) => {
                handle_component_interaction(&ctx, component).await;
            },
            _ => {},
        }
    }
}

async fn handle_component_interaction(ctx: &Context, component: ComponentInteraction) {
    if component.data.custom_id.starts_with("standings_") {
        handle_standings_interaction(ctx, component).await;
        return;
    }

    let response = component.data.custom_id.as_str();

    let data = CreateInteractionResponseMessage::new()
        .content(response)
        .ephemeral(true);

    if let Err(why) = component.create_response(&ctx.http, CreateInteractionResponse::Message(data)).await {
        info!("Cannot respond to component interaction: {}", why);
    }
}

async fn handle_standings_interaction(ctx: &Context, component: ComponentInteraction) {
    let parts: Vec<&str> = component.data.custom_id.split('_').collect();
    if parts.len() < 3 {
        return;
    }
    // parts formatted like ("standings_prev_{}", page)
    let action = parts[1];
    let current_page: usize = parts[2].parse().unwrap_or(0);

    // extract league_id from the embed footer
    let league_id = component.message.embeds.first()
        .and_then(|embed| embed.footer.as_ref())
        .and_then(|footer| footer.text.split("League ID: ").nth(1))
        .and_then(|s| s.split(' ').next())
        .and_then(|s| s.parse::<i32>().ok());    

    let Some(league_id) = league_id else {
        let data = CreateInteractionResponseMessage::new()
            .content("Error: Could not determine league ID")
            .ephemeral(true);
        let _ = component.create_response(&ctx.http, CreateInteractionResponse::Message(data)).await;
        return;
    };

    // Calculate new page and determine if we need new API data
    let new_page = match action {
        "prev" => current_page.saturating_sub(1),
        "next" => current_page + 1,
        "refresh" => current_page,
        _ => current_page,
    };

    let needed_api_page = ((new_page / 2) + 1) as i32;
    let standings_result = fpl::models::league::LeagueStandings::fetch_page(league_id, Some(needed_api_page)).await;

    match standings_result {
        Ok(standings) => {
            let per_page = 25;
            let total_managers = standings.standings.managers.len();
            let max_page = (50 * (needed_api_page as usize) + total_managers - 1) / per_page ;
            let actual_page = new_page.min(max_page);

            let embed = commands::standings::build_standings_embed(&standings, actual_page);
            let buttons = commands::standings::build_navigation_buttons(actual_page, &standings);

            let response = CreateInteractionResponse::UpdateMessage(
                serenity::builder::CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .button(buttons.prev)
                    .button(buttons.next)
                    .button(buttons.refresh)
            );

            if let Err(why) = component.create_response(&ctx.http, response).await {
                info!("Cannot update standings message: {}", why);
            }
        },
        Err(e) => {
            let data = CreateInteractionResponseMessage::new()
                .content(format!("Error fetching standings: {}", e))
                .ephemeral(true);
            let _ = component.create_response(&ctx.http, CreateInteractionResponse::Message(data)).await;
        }
    }
}
