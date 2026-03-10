#![allow(unused_imports)]
use anyhow::{anyhow, Result};
use log::{error, info};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, ResolvedOption, ResolvedValue,
};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::CommandOptionType;

use crate::database::models::DBChannel;
use crate::database::{models::DBUser, service::db_service};
use crate::utils::fpl_client::fpl_client;
use crate::utils::type_conversion::r_option_to_i32;

pub fn register() -> CreateCommand {
    CreateCommand::new("check_team")
        .description("Check what players are in a manager's team")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "manager_id", "FPL Manager ID")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "Discord User to check")
                .required(false),
        )
}

pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
) -> Result<CreateInteractionResponse> {
    let db = db_service();

    // Parse options first to see which one was provided
    let options = command.data.options();
    let manager_id_opt = options.iter().find(|opt| opt.name == "manager_id");
    let user_opt = options.iter().find(|opt| opt.name == "user");

    let manager_id = match (manager_id_opt, user_opt) {
        (
            Some(ResolvedOption {
                value: ResolvedValue::Integer(id),
                ..
            }),
            _,
        ) => *id as i32,
        (
            _,
            Some(ResolvedOption {
                value: ResolvedValue::User(user, _),
                ..
            }),
        ) => {
            let db_user: DBUser = db.get_user(user.id).await?;
            match db_user.manager_id {
                Some(id) => id,
                _ => {
                    return Ok(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(format!(
                            "User {} has not linked their FPL manager ID.",
                            user.name
                        )),
                    ));
                }
            }
        }
        _ => {
            let user: DBUser = db.get_user(command.user.id).await?;
            match user.manager_id {
                Some(id) => id,
                _ => {
                    return Ok(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(
                            "No manager_id configured for you. Use /update_manager_id please!",
                        ),
                    ));
                }
            }
        }
    };

    let bootstrap = fpl_client().get_bootstrap().await?;
    let current_gw = bootstrap
        .events
        .iter()
        .find(|e| e.is_current)
        .map(|e| e.id)
        .ok_or_else(|| anyhow!("No current gameweek found"))?;

    let picks = fpl_client().get_manager_picks(manager_id, current_gw).await?;
    let manager = fpl_client().get_manager(manager_id).await?;

    let manager_name = format!(
        "{} {}",
        manager.player_first_name, manager.player_last_name
    );
    let team_name = manager.name.clone();

    let embed = build_team_embed(
        &picks,
        &manager_name,
        &team_name,
        &bootstrap.elements,
        &bootstrap.teams,
        current_gw,
        manager_id,
    );

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}

fn build_team_embed(
    picks_data: &fpl_client::ManagerPicks,
    manager_name: &str,
    team_name: &str,
    elements: &[fpl_client::models::bootstrap_static::Element],
    teams: &[fpl_client::models::bootstrap_static::Team],
    gameweek: i32,
    manager_id: i32,
) -> CreateEmbed {
    let mut starters = Vec::new();
    let mut bench = Vec::new();

    for pick in &picks_data.picks {
        let player_opt = elements.iter().find(|e| e.id == pick.element);
        if let Some(player) = player_opt {
            let team_opt = teams.iter().find(|t| t.id == player.team);

            let web_name = &player.web_name;
            let team_short = team_opt.map(|t| t.short_name.as_str()).unwrap_or("???");
            let event_points = player.event_points;

            let mut display_name = web_name.clone();
            if pick.is_captain {
                display_name = format!("{} (C)", display_name);
            } else if pick.is_vice_captain {
                display_name = format!("{} (V)", display_name);
            }

            let player_info = (
                display_name,
                team_short.to_string(),
                event_points * pick.multiplier,
            );

            if pick.multiplier > 0 {
                starters.push(player_info);
            } else {
                bench.push(player_info);
            }
        }
    }

    let gw_points = picks_data.entry_history.points;
    let total_points = picks_data.entry_history.total_points;
    let overall_rank = picks_data.entry_history.overall_rank.unwrap_or(0);
    let rank = picks_data.entry_history.rank.unwrap_or(0);

    let mut description = String::new();

    if let Some(chip) = &picks_data.active_chip {
        description.push_str(&format!("**Active Chip:** {}\n\n", chip));
    }

    let max_name_len = starters
        .iter()
        .chain(bench.iter())
        .map(|(n, _, _)| n.len())
        .max()
        .unwrap_or(10)
        .min(15);

    description.push_str("**Starting XI**\n```\n");
    for (name, team, points) in &starters {
        description.push_str(&format!(
            "{:<name_width$} {:<3} {:>3}pts\n",
            name,
            team,
            points,
            name_width = max_name_len
        ));
    }
    description.push_str("```\n");

    if !bench.is_empty() {
        description.push_str("**Bench**\n```\n");
        for (name, team, points) in &bench {
            description.push_str(&format!(
                "{:<name_width$} {:<3} {:>3}pts\n",
                name,
                team,
                points,
                name_width = max_name_len
            ));
        }
        description.push_str("```");
    }

    CreateEmbed::new()
        .title(format!("{} - GW{}", team_name, gameweek))
        .description(description)
        .color(0x37003c)
        .field("GW Points", gw_points.to_string(), true)
        .field("Total Points", total_points.to_string(), true)
        .field("Overall Rank", overall_rank.to_string(), true)
        .field("GW Rank", rank.to_string(), true)
        .footer(serenity::builder::CreateEmbedFooter::new(format!(
            "Manager: {} • ID: {}",
            manager_name, manager_id
        )))
}
