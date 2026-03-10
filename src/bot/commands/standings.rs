use crate::database::service::db_service;
use anyhow::{anyhow, Result};
use fpl_client::client::FplApiClient;
use fpl_client::models::league::StandingEntry;
use fpl_client::LeagueStandings;
use log::{error, info};
use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::{CreateButton, CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use std::borrow::Cow;

/// Main handler for the `/standings` slash command
pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let user_id = &command.user.name;
    info!("Processing standings command for user {}", user_id);

    let league_id = if command.data.options().is_empty() {
        let db = db_service();
        let channel = db.get_channel(command.channel_id).await?;

        channel
            .default_league_id
            .ok_or_else(|| anyhow!("No default league"))?
    } else {
        extract_league_id(&command.data.options())?
    };

    info!(
        "Fetching standings for league_id: {} requested by user {}",
        league_id, user_id
    );

    let client = FplApiClient::new()?;
    let standings = match client.get_league_standings(league_id).await {
        Ok(standings) => {
            info!(
                "Successfully fetched standings for league_id: {} (user {})",
                league_id, user_id
            );
            standings
        }
        Err(e) => {
            error!(
                "Failed to fetch standings for league_id: {} (user {}): {}",
                league_id, user_id, e
            );
            return Err(e);
        }
    };

    let page = 0;
    let embed = build_standings_embed(&standings, page);
    let buttons = build_navigation_buttons(page, &standings);

    info!(
        "Successfully built standings response for league_id: {} (user {})",
        league_id, user_id
    );
    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .button(buttons.prev)
            .button(buttons.next)
            .button(buttons.refresh),
    ))
}

/// Extracts league ID from Discord command options
fn extract_league_id(options: &[ResolvedOption]) -> Result<i32> {
    match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(id),
            ..
        }) => Ok(*id as i32),
        _ => {
            error!("No valid league_id provided in command options");
            Err(anyhow!("Please provide a valid league ID"))
        }
    }
}

/// Builds a Discord embed displaying league standings
pub fn build_standings_embed(standings: &LeagueStandings, page: usize) -> CreateEmbed {
    let managers = &standings.standings.results;
    let per_page = 25;
    let start_idx = (page * per_page) % 50;
    let end_idx = (start_idx + 25).min(managers.len());
    let page_managers = &managers[start_idx..end_idx];

    let max_rank_width = managers
        .iter()
        .map(|m| number_len(m.rank))
        .max()
        .unwrap_or(2);

    let max_change_width = managers
        .iter()
        .map(|m| number_len(-(m.rank - m.last_rank)))
        .max()
        .unwrap_or(4)
        + 3;

    let max_points_width = managers
        .iter()
        .map(|m| number_len(m.total))
        .max()
        .unwrap_or(4);

    let max_gw_width = managers
        .iter()
        .map(|m| number_len(m.event_total))
        .max()
        .unwrap_or(4)
        + 2;

    let separators_width: usize = 7;
    let fixed_width =
        max_rank_width + max_change_width + max_points_width + max_gw_width + separators_width;
    let total_available: usize = 40;
    let name_width = total_available.saturating_sub(fixed_width).max(5);

    let mut description = String::new();
    description.push_str("```");
    for manager in page_managers.iter() {
        let name = format_name(manager, name_width);
        let rank_diff = -(manager.rank - manager.last_rank);

        description.push_str(&format!(
            "#{rank:<rank_width$}{change:<change_width$}| {name:<name_width$} | {total:<points_width$} {gw:<gw_width$}pts\n",
            rank = manager.rank,
            change = format!("({:+})", rank_diff),
            name = name,
            total = manager.total,
            gw = format!("({})", manager.event_total),
            rank_width = max_rank_width,
            change_width = max_change_width,
            name_width = name_width,
            points_width = max_points_width,
            gw_width = max_gw_width
        ));
    }

    description.push_str("```");

    let total_pages = if standings.standings.has_next {
        "?".to_string()
    } else {
        format!("{}", (managers.len() + per_page - 1) / per_page)
    };

    CreateEmbed::new()
        .title(format!("🏆  {}", standings.league.name))
        .description(description)
        .color(0x37003c)
        .footer(serenity::builder::CreateEmbedFooter::new(format!(
            "League ID: {} • Page {} of {}",
            standings.league.id,
            page + 1,
            total_pages
        )))
}

/// Navigation button configuration for standings pagination
pub struct NavigationButtons {
    pub prev: CreateButton,
    pub next: CreateButton,
    pub refresh: CreateButton,
}

/// Creates navigation buttons for standings pagination
pub fn build_navigation_buttons(page: usize, standings: &LeagueStandings) -> NavigationButtons {
    let per_page = 25;
    let total_managers = standings.standings.results.len();
    let api_has_next = standings.standings.has_next;
    let total_pages_current = (total_managers + per_page - 1) / per_page;
    let has_prev = page > 0;
    let has_next = page + 1 < total_pages_current || api_has_next;

    NavigationButtons {
        prev: CreateButton::new(format!("standings_prev_{}", page))
            .label("⬅️ Previous")
            .style(ButtonStyle::Secondary)
            .disabled(!has_prev),
        next: CreateButton::new(format!("standings_next_{}", page))
            .label("Next ➡️")
            .style(ButtonStyle::Secondary)
            .disabled(!has_next),
        refresh: CreateButton::new(format!("standings_refresh_{}", page))
            .label("🔄 Refresh")
            .style(ButtonStyle::Primary),
    }
}

/// Registers the standings command with Discord
pub fn register() -> CreateCommand {
    CreateCommand::new("standings")
        .description("Get FPL league standings")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "league_id", "The FPL league ID")
                .required(false),
        )
}

/// Formats manager name to fit within specified width
fn format_name(manager: &'_ StandingEntry, name_width: usize) -> Cow<'_, str> {
    let name: Cow<str> = if manager.player_name.chars().count() <= name_width {
        Cow::Borrowed(&manager.player_name)
    } else {
        let (first_name, last_name) = manager
            .player_name
            .split_once(" ")
            .unwrap_or((&manager.player_name, ""));
        let truncated = format!(
            "{} {}.",
            first_name,
            last_name.chars().next().unwrap_or(' ')
        );
        if truncated.chars().count() <= name_width {
            Cow::Owned(truncated)
        } else {
            let first_only: String = first_name
                .chars()
                .take(name_width.saturating_sub(1))
                .collect();
            Cow::Owned(format!("{}.", first_only))
        }
    };
    name
}

/// Calculates the character width needed to display a number
fn number_len(mut num: i32) -> usize {
    let mut count = 0;
    if num <= 0 {
        num *= -1;
        count += 1;
    }
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}
