use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ButtonStyle};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateButton};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use crate::fpl::models::league::{LeagueStandings, StandingsManager};
use crate::fpl::models::teams::get_team_name;
use anyhow::{Result, anyhow};

use crate::fpl::models::fixtures::{fetch_fixtures, GameweekFixtures};


pub fn register() -> CreateCommand {
    CreateCommand::new("fixtures")
        .description("Get a given weeks fixtures")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "gameweek",
                "The gameweek number"
            ).required(true)
        )
}

pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
    let week = extract_gameweek(command)?;
    let fixtures = fetch_fixtures(week).await?;

    let embed = build_fixtures_embed(&fixtures);

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
    ))
}

fn extract_gameweek(command: &CommandInteraction) -> Result<i32>{
    let resolved = command.data.options();
    match resolved.first(){
        Some(ResolvedOption{ value: ResolvedValue::Integer(id), ..}) => Ok(*id as i32),
        _ => Err(anyhow!("Please provide a valid gameweek"))
    }
}

fn build_fixtures_embed(fixtures: &GameweekFixtures) -> CreateEmbed{
    let mut description = String::new();
    description.push_str("```");
    for fixture in fixtures.fixtures.iter(){
        let home_team = get_team_name(fixture.team_h).name;
        let away_team = get_team_name(fixture.team_a).name;

        description.push_str(format!("{:^38}\n{:>17} - {:<18}\n\n", 
            fixture.kickoff_time.format("%d.%m %H:%M"),
            home_team, 
            away_team, 
        ).as_str());
    }
    description.push_str("```");

    CreateEmbed::new()
        .title(format!("Gameweek {}", fixtures.gameweek))
        .description(description)
        .color(0x37003c) // purple
}
