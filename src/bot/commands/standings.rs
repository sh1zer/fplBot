//! League standings command implementation
//!
//! Provides Discord slash command functionality for displaying FPL league standings
//! with interactive pagination and navigation controls.

use std::borrow::Cow;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ButtonStyle};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateButton};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use crate::fpl::models::league::{LeagueStandings, StandingsManager};
use anyhow::{Result, anyhow};
use tracing::{info, error};

/// Main handler for the `/standings` slash command
///
/// Fetches and displays league standings for a given league ID with interactive
/// pagination controls. Shows manager names, positions, and points in an embed format.
///
/// # Arguments
/// * `_ctx` - Discord context (unused in current implementation)
/// * `command` - The slash command interaction containing user input
///
/// # Returns
/// * `Result<CreateInteractionResponse>` - Discord response with standings embed and navigation buttons
///
/// # Errors
/// Returns error if:
/// - League ID is not provided or invalid
/// - FPL API request fails
/// - League data cannot be processed
///
/// # Example Usage
/// `/standings league_id:123456`
pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
    let user_id = &command.user.name;
    info!("Processing standings command for user {}", user_id);
    
    let league_id = extract_league_id(&command.data.options())?;
    info!("Fetching standings for league_id: {} requested by user {}", league_id, user_id);
    
    let standings = match LeagueStandings::fetch(league_id).await {
        Ok(standings) => {
            info!("Successfully fetched standings for league_id: {} (user {})", league_id, user_id);
            standings
        }
        Err(e) => {
            error!("Failed to fetch standings for league_id: {} (user {}): {}", league_id, user_id, e);
            return Err(e);
        }
    };

    let page = 0;
    let embed = build_standings_embed(&standings, page);
    let buttons = build_navigation_buttons(page, &standings);

    info!("Successfully built standings response for league_id: {} (user {})", league_id, user_id);
    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .button(buttons.prev)
            .button(buttons.next)
            .button(buttons.refresh)
    ))
}

/// Extracts league ID from Discord command options
///
/// Parses the first command option to extract the league ID integer value.
///
/// # Arguments
/// * `options` - Array of resolved command options from Discord
///
/// # Returns
/// * `Result<i32>` - The league ID as a 32-bit integer
///
/// # Errors
/// Returns error if no valid integer option is provided
fn extract_league_id(options: &[ResolvedOption]) -> Result<i32> {
    match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(id), ..
        }) => {
            // info!("Extracted league_id: {}", id);
            Ok(*id as i32)
        }
        _ => {
            error!("No valid league_id provided in command options");
            Err(anyhow!("Please provide a valid league ID"))
        }
    }
}

/// Builds a Discord embed displaying league standings
///
/// Creates a formatted embed with standings data, including manager names, ranks,
/// points, and gameweek performance. Uses fixed-width formatting for consistent alignment.
///
/// # Arguments
/// * `standings` - The league standings data from FPL API
/// * `page` - Current page number for pagination (0-based)
///
/// # Returns
/// * `CreateEmbed` - Discord embed with formatted standings table
///
/// # Display Format
/// Shows columns for: Rank, Change, Manager Name, Total Points, GW Points
/// Uses code block formatting for monospace alignment
pub fn build_standings_embed(standings: &LeagueStandings, page: usize) -> CreateEmbed {
    let managers = &standings.standings.managers;
    let per_page = 25;
    let start_idx = (page * per_page) % 50;
    let end_idx = (start_idx + 25).min(managers.len());
    let page_managers = &managers[start_idx..end_idx];
 
    // Calculate maximum widths needed for each column (from all managers for consistency)
    let max_rank_width = managers.iter()
        .map(|m| number_len(m.current_rank))
        .max()
        .unwrap_or(2);

    let max_change_width = managers.iter()
        .map(|m| number_len(-(m.current_rank - m.previous_rank)))
        .max()
        .unwrap_or(4) + 3;

    let max_points_width = managers.iter()
        .map(|m| number_len(m.total_points))
        .max()
        .unwrap_or(4);

    let max_gw_width = managers.iter()
        .map(|m| number_len(m.gameweek_points))
        .max()
        .unwrap_or(4) + 2;

    let separators_width: usize = 7;
    let fixed_width = max_rank_width + max_change_width + max_points_width + max_gw_width + separators_width;
    let total_available: usize = 40;
    let name_width = total_available.saturating_sub(fixed_width).max(5); // minimum 5 chars for names

    let mut description = String::new();
    description.push_str("```");
    // description.push_str(format!("{}{}{}{}", max_rank_width, max_change_width, max_points_width, max_gw_width).as_str());
    for manager in page_managers.iter() {
        let name = format_name(manager, name_width);
        let rank_diff = -(manager.current_rank - manager.previous_rank);

        description.push_str(&format!(
            "#{rank:<rank_width$}{change:<change_width$}| {name:<name_width$} | {total:<points_width$} {gw:<gw_width$}pts\n",
            rank = manager.current_rank,
            change = format!("({:+})", rank_diff),
            name = name,
            total = manager.total_points,
            gw = format!("({})", manager.gameweek_points),
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
        .title(format!("🏆  {}", standings.league_info.league_name))
        .description(description)
        .color(0x37003c) // purple
        .footer(serenity::builder::CreateEmbedFooter::new(format!(
            "League ID: {} • Page {} of {}",
            standings.league_info.id,
            page + 1,
            total_pages
        )))
}

/// Navigation button configuration for standings pagination
///
/// Holds the three main navigation buttons used in standings displays.
pub struct NavigationButtons {
    /// Previous page button
    pub prev: CreateButton,
    /// Next page button  
    pub next: CreateButton,
    /// Refresh current page button
    pub refresh: CreateButton,
}

/// Creates navigation buttons for standings pagination
///
/// Builds previous, next, and refresh buttons with appropriate enabled/disabled states
/// based on current page position and available data.
///
/// # Arguments
/// * `page` - Current page number (0-based)
/// * `standings` - League standings data to determine pagination limits
///
/// # Returns
/// * `NavigationButtons` - Struct containing the three navigation buttons
///
/// # Button Behavior
/// - Previous: Disabled on first page
/// - Next: Disabled on last page (when no more data available)
/// - Refresh: Always enabled
pub fn build_navigation_buttons(page: usize, standings: &LeagueStandings) -> NavigationButtons {
    let per_page = 25;
    let total_managers = standings.standings.managers.len();
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
///
/// Creates the command definition for the `/standings` slash command with required
/// league_id parameter.
///
/// # Returns
/// * `CreateCommand` - Discord command definition ready for registration
pub fn register() -> CreateCommand {
    CreateCommand::new("standings")
        .description("Get FPL league standings")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "league_id",
                "The FPL league ID"
            ).required(true)
        )
}

/// Formats manager name to fit within specified width
///
/// Truncates long manager names using intelligent strategies:
/// 1. Use full name if it fits
/// 2. Use "First L." format if shorter
/// 3. Use "First." if still too long
///
/// # Arguments
/// * `manager` - The manager data containing the name
/// * `name_width` - Maximum character width allowed
///
/// # Returns
/// * `Cow<str>` - Formatted name that fits within the width constraint
fn format_name(manager: &StandingsManager, name_width: usize) -> Cow<str>{
    let name: Cow<str> = if manager.manager_name.chars().count() <= name_width {
        Cow::Borrowed(&manager.manager_name)
    } else {
        let (first_name, last_name) = manager.manager_name.split_once(" ").unwrap_or((&manager.manager_name, "")); 
        // creates truncated name like "John S."
        let truncated = format!("{} {}.", first_name, last_name.chars().next().unwrap_or(' '));
        if truncated.chars().count() <= name_width {
            Cow::Owned(truncated)
        } else {
            // if truncated too long take first name
            let first_only: String = first_name.chars().take(name_width.saturating_sub(1)).collect();
            Cow::Owned(format!("{}.", first_only))
        }
    };
    name
}

/// Calculates the character width needed to display a number
///
/// Counts the number of digits plus space for negative sign if applicable.
/// Used for calculating column widths in standings tables.
///
/// # Arguments
/// * `num` - The integer to measure
///
/// # Returns
/// * `usize` - Number of characters needed to display the number
fn number_len(mut num: i32) -> usize{
    let mut count = 0;
    if num <= 0{
        num *= -1;
        count += 1;
    }
    while num > 0{
        num /= 10;
        count += 1;
    }
    count
}

