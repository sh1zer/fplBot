//! Fixtures command implementation
//!
//! Provides Discord slash command functionality for displaying FPL gameweek fixtures
//! with match details, scores, and team information.

use crate::fpl::models::teams::get_team_name;
use anyhow::{anyhow, Result};
use log::{error, info};
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::fpl::models::fixtures::{fetch_fixtures, GameweekFixtures};

/// Registers the fixtures command with Discord
///
/// Creates the command definition for the `/fixtures` slash command with required
/// gameweek parameter.
///
/// # Returns
/// * `CreateCommand` - Discord command definition ready for registration
pub fn register() -> CreateCommand {
    CreateCommand::new("fixtures")
        .description("Get a given weeks fixtures")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "gameweek",
                "The gameweek number",
            )
            .required(true),
        )
}

/// Main handler for the `/fixtures` slash command
///
/// Fetches and displays FPL fixtures for a specific gameweek with match details,
/// scores, and kickoff times in an embed format.
///
/// # Arguments
/// * `_ctx` - Discord context (unused in current implementation)
/// * `command` - The slash command interaction containing user input
///
/// # Returns
/// * `Result<CreateInteractionResponse>` - Discord response with fixtures embed
///
/// # Errors
/// Returns error if:
/// - Gameweek number is not provided or invalid
/// - FPL API request fails
/// - Fixture data cannot be processed
///
/// # Example Usage
/// `/fixtures gameweek:1`
pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let user_id = &command.user.name;
    info!("Processing fixtures command for user {}", user_id);

    let week = extract_gameweek(command)?;
    info!(
        "Fetching fixtures for gameweek {} requested by user {}",
        week, user_id
    );

    let fixtures = match fetch_fixtures(week).await {
        Ok(fixtures) => {
            info!(
                "Successfully fetched {} fixtures for gameweek {} (user {})",
                fixtures.fixtures.len(),
                week,
                user_id
            );
            fixtures
        }
        Err(e) => {
            error!(
                "Failed to fetch fixtures for gameweek {} (user {}): {}",
                week, user_id, e
            );
            return Err(e);
        }
    };

    let embed = build_fixtures_embed(&fixtures);

    info!(
        "Successfully built fixtures response for gameweek {} (user {})",
        week, user_id
    );
    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}

/// Extracts gameweek number from Discord command options
///
/// Parses the first command option to extract the gameweek integer value.
///
/// # Arguments
/// * `command` - The Discord command interaction containing options
///
/// # Returns
/// * `Result<i32>` - The gameweek number as a 32-bit integer
///
/// # Errors
/// Returns error if no valid integer option is provided
fn extract_gameweek(command: &CommandInteraction) -> Result<i32> {
    let resolved = command.data.options();
    match resolved.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(id),
            ..
        }) => {
            // info!("Extracted gameweek: {}", id);
            Ok(*id as i32)
        }
        _ => {
            error!("No valid gameweek provided in command options");
            Err(anyhow!("Please provide a valid gameweek"))
        }
    }
}

/// Builds a Discord embed displaying gameweek fixtures
///
/// Creates a formatted embed with fixture data, including team names and kickoff times.
/// Uses fixed-width formatting for consistent alignment in a code block.
///
/// # Arguments
/// * `fixtures` - The gameweek fixtures data from FPL API
///
/// # Returns
/// * `CreateEmbed` - Discord embed with formatted fixtures list
///
/// # Display Format
/// Shows each fixture with kickoff time centered and team names aligned
/// in the format: "Date Time\nHome Team - Away Team"
fn build_fixtures_embed(fixtures: &GameweekFixtures) -> CreateEmbed {
    let mut description = String::new();
    description.push_str("```");
    for fixture in fixtures.fixtures.iter() {
        let home_team = get_team_name(fixture.team_h).name;
        let away_team = get_team_name(fixture.team_a).name;

        description.push_str(
            format!(
                "{:^38}\n{:>17} - {:<18}\n\n",
                fixture.kickoff_time.format("%d.%m %H:%M"),
                home_team,
                away_team,
            )
            .as_str(),
        );
    }
    description.push_str("```");

    CreateEmbed::new()
        .title(format!("Gameweek {}", fixtures.gameweek))
        .description(description)
        .color(0x37003c) // purple
}
