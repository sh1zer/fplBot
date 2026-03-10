use anyhow::{anyhow, Result};
use fpl_client::models::bootstrap_static::Team;
use fpl_client::models::fixture::Fixture;
use log::{error, info};
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

use crate::utils::fpl_client::fpl_client;

/// Registers the fixtures command with Discord
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

    let fixtures: Vec<Fixture> = match fpl_client().get_fixtures(Some(week), None).await {
        Ok(fixtures) => {
            info!(
                "Successfully fetched {} fixtures for gameweek {} (user {})",
                fixtures.len(),
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

    // this is horrible but i dont really feel like doing it correct right now
    let bootstrap = fpl_client().get_bootstrap().await?;
    let teams = bootstrap.teams;

    let embed = build_fixtures_embed(&fixtures, &teams, week);

    info!(
        "Successfully built fixtures response for gameweek {} (user {})",
        week, user_id
    );
    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}

/// Extracts gameweek number from Discord command options
fn extract_gameweek(command: &CommandInteraction) -> Result<i32> {
    let resolved = command.data.options();
    match resolved.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(id),
            ..
        }) => Ok(*id as i32),
        _ => {
            error!("No valid gameweek provided in command options");
            Err(anyhow!("Please provide a valid gameweek"))
        }
    }
}

/// Looks up a team name by ID from a list of teams
fn get_team_name(teams: &[Team], team_id: i32) -> &str {
    teams
        .iter()
        .find(|t| t.id == team_id)
        .map(|t| t.name.as_str())
        .unwrap_or("Unknown")
}

/// Builds a Discord embed displaying gameweek fixtures
fn build_fixtures_embed(fixtures: &[Fixture], teams: &[Team], gameweek: i32) -> CreateEmbed {
    let mut description = String::new();
    description.push_str("```");
    for fixture in fixtures.iter() {
        let home_team = get_team_name(teams, fixture.team_h);
        let away_team = get_team_name(teams, fixture.team_a);
        let kickoff = fixture.kickoff_time.as_deref().unwrap_or("TBD");
        // Trim to "dd.mm HH:MM" if possible, otherwise use as-is
        let kickoff_display = if kickoff.len() >= 16 {
            // ISO format: "2024-08-16T20:00:00Z" -> "16.08 20:00"
            let date_part = &kickoff[..10]; // "2024-08-16"
            let time_part = &kickoff[11..16]; // "20:00"
            let parts: Vec<&str> = date_part.split('-').collect();
            if parts.len() == 3 {
                format!("{}.{} {}", parts[2], parts[1], time_part)
            } else {
                kickoff.to_string()
            }
        } else {
            kickoff.to_string()
        };

        description.push_str(
            format!(
                "{:^38}\n{:>17} - {:<18}\n\n",
                kickoff_display, home_team, away_team,
            )
            .as_str(),
        );
    }
    description.push_str("```");

    CreateEmbed::new()
        .title(format!("Gameweek {}", gameweek))
        .description(description)
        .color(0x37003c) // purple
}
