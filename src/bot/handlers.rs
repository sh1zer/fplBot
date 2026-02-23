//! Discord event handlers for the FPL bot
//!
//! This module contains the main event handler implementation that processes
//! Discord events, including command interactions and component interactions.
//! It handles bot initialization, command registration, and routing of user
//! interactions to appropriate command handlers.

use serenity::{
    all::{
        ComponentInteraction, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, GuildId, Interaction, Ready,
    },
    async_trait,
    prelude::*,
};
use tracing::info;

use crate::{bot::commands, fpl};

/// Main event handler for the Discord bot
///
/// Implements the Serenity [`EventHandler`] trait to process Discord events.
/// Handles bot initialization, slash command registration, and interaction routing.
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Called when the bot successfully connects to Discord
    ///
    /// Registers all available slash commands globally and logs the bot's connection status.
    ///
    /// # Arguments
    /// * `ctx` - The Discord context for making API calls
    /// * `ready` - Information about the bot's ready state
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);

        let commands = vec![
            CreateCommand::new("hello").description("Say hello to the bot"),
            commands::standings::register(),
            commands::fixtures::register(),
            commands::update_manager_id::register(),
            commands::check_manager_id::register(),
            commands::update_channel_league_id::register(),
        ];
        let guild_id = GuildId::new(1221876813165363270); // Replace with your server's ID
        match guild_id.set_commands(&ctx.http, commands).await {
            // match Command::set_global_commands(&ctx.http, commands).await {
            Ok(_) => info!("Successfully registered slash commands"),
            Err(e) => info!("Failed to register slash commands: {}", e),
        }
    }

    /// Handles incoming Discord interactions (commands and components)
    ///
    /// Routes slash commands to their respective handlers and processes
    /// component interactions (button clicks, select menus, etc.).
    ///
    /// # Arguments
    /// * `ctx` - The Discord context for making API calls
    /// * `interaction` - The interaction data from Discord
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let response = match command.data.name.as_str() {
                    "hello" => {
                        let data = CreateInteractionResponseMessage::new().content("Hey!");
                        Ok(CreateInteractionResponse::Message(data))
                    }
                    "standings" => commands::standings::run(&ctx, &command).await,
                    "fixtures" => commands::fixtures::run(&ctx, &command).await,
                    "update_manager_id" => commands::update_manager_id::run(&ctx, &command).await,
                    "check_manager_id" => commands::check_manager_id::run(&ctx, &command).await,
                    _ => {
                        let data =
                            CreateInteractionResponseMessage::new().content("Unknown command");
                        Ok(CreateInteractionResponse::Message(data))
                    }
                };

                if let Err(why) = match response {
                    Ok(response) => command.create_response(&ctx.http, response).await,
                    Err(e) => {
                        let error_response = CreateInteractionResponseMessage::new()
                            .content(format!("Error: {}", e));
                        command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(error_response),
                            )
                            .await
                    }
                } {
                    info!("Cannot respond to slash command: {}", why);
                }
            }
            Interaction::Component(component) => {
                handle_component_interaction(&ctx, component).await;
            }
            _ => {}
        }
    }
}

/// Handles component interactions (buttons, select menus, etc.)
///
/// Routes component interactions based on their custom ID to appropriate handlers.
/// Currently supports standings navigation buttons and other general components.
///
/// # Arguments
/// * `ctx` - The Discord context for making API calls
/// * `component` - The component interaction data
async fn handle_component_interaction(ctx: &Context, component: ComponentInteraction) {
    if component.data.custom_id.starts_with("standings_") {
        handle_standings_interaction(ctx, component).await;
        return;
    }

    let response = component.data.custom_id.as_str();

    let data = CreateInteractionResponseMessage::new()
        .content(response)
        .ephemeral(true);

    if let Err(why) = component
        .create_response(&ctx.http, CreateInteractionResponse::Message(data))
        .await
    {
        info!("Cannot respond to component interaction: {}", why);
    }
}

/// Handles standings-specific component interactions
///
/// Processes navigation buttons (previous/next page) for league standings displays.
/// Extracts pagination information from the component custom ID and updates the
/// standings display with the requested page.
///
/// # Arguments
/// * `ctx` - The Discord context for making API calls
/// * `component` - The component interaction data with standings-specific custom ID
///
/// # Custom ID Format
/// Expected format: `standings_{action}_{current_page}` where:
/// - `action` can be "prev", "next", or "refresh"
/// - `current_page` is the 0-based page number
async fn handle_standings_interaction(ctx: &Context, component: ComponentInteraction) {
    let parts: Vec<&str> = component.data.custom_id.split('_').collect();
    if parts.len() < 3 {
        return;
    }
    // parts formatted like ("standings_prev_{}", page)
    let action = parts[1];
    let current_page: usize = parts[2].parse().unwrap_or(0);

    // extract league_id from the embed footer
    let league_id = component
        .message
        .embeds
        .first()
        .and_then(|embed| embed.footer.as_ref())
        .and_then(|footer| footer.text.split("League ID: ").nth(1))
        .and_then(|s| s.split(' ').next())
        .and_then(|s| s.parse::<i32>().ok());

    let Some(league_id) = league_id else {
        let data = CreateInteractionResponseMessage::new()
            .content("Error: Could not determine league ID")
            .ephemeral(true);
        let _ = component
            .create_response(&ctx.http, CreateInteractionResponse::Message(data))
            .await;
        return;
    };

    let new_page = match action {
        "prev" => current_page.saturating_sub(1),
        "next" => current_page + 1,
        _ => current_page,
    };

    let needed_api_page = ((new_page / 2) + 1) as i32;
    let standings_result =
        fpl::models::league::LeagueStandings::fetch_page(league_id, Some(needed_api_page)).await;

    match standings_result {
        Ok(standings) => {
            let per_page = 25;
            let total_managers = standings.standings.managers.len();
            let max_page = (50 * (needed_api_page as usize) + total_managers - 1) / per_page;
            let actual_page = new_page.min(max_page);

            let embed = commands::standings::build_standings_embed(&standings, actual_page);
            let buttons = commands::standings::build_navigation_buttons(actual_page, &standings);

            let response = CreateInteractionResponse::UpdateMessage(
                serenity::builder::CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .button(buttons.prev)
                    .button(buttons.next)
                    .button(buttons.refresh),
            );

            if let Err(why) = component.create_response(&ctx.http, response).await {
                info!("Cannot update standings message: {}", why);
            }
        }
        Err(e) => {
            let data = CreateInteractionResponseMessage::new()
                .content(format!("Error fetching standings: {}", e))
                .ephemeral(true);
            let _ = component
                .create_response(&ctx.http, CreateInteractionResponse::Message(data))
                .await;
        }
    }
}
