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
use crate::fpl::fpl_client;
use crate::fpl::models::manager;
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
                None => {
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
                None => {
                    return Ok(CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(
                            "No manager_id configured for you. Use /update_manager_id please!",
                        ),
                    ));
                }
            }
        }
    };

    let current_gw = fpl_client().get_current_gameweek_id().await?;
    let team_picks = fpl_client()
        .get_manager_team(manager_id, current_gw)
        .await?;

    let manager_summary = fpl_client().get_manager_summary(manager_id).await?;
    let player_first_name = manager_summary["player_first_name"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let player_last_name = manager_summary["player_last_name"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let manager_name = format!("{} {}", player_first_name, player_last_name);
    let team_name = manager_summary["name"]
        .as_str()
        .unwrap_or("FPL Team")
        .to_string();

    let general_data = fpl_client().get_general().await?;
    let embed = build_team_embed(
        &team_picks,
        &manager_name,
        &team_name,
        &general_data,
        current_gw,
        manager_id,
    );

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}

fn build_team_embed(
    picks_data: &serde_json::Value,
    manager_name: &str,
    team_name: &str,
    general_data: &serde_json::Value,
    gameweek: i32,
    manager_id: i32,
) -> CreateEmbed {
    let picks = picks_data["picks"].as_array().unwrap();
    let entry_history = &picks_data["entry_history"];
    let active_chip = picks_data["active_chip"].as_str();

    let elements = general_data["elements"].as_array().unwrap();
    let teams = general_data["teams"].as_array().unwrap();

    let mut starters = Vec::new();
    let mut bench = Vec::new();

    for pick in picks {
        let element_id = pick["element"].as_i64().unwrap_or(0);
        let is_captain = pick["is_captain"].as_bool().unwrap_or(false);
        let is_vice = pick["is_vice_captain"].as_bool().unwrap_or(false);
        let multiplier = pick["multiplier"].as_i64().unwrap_or(1);

        let player_opt = elements
            .iter()
            .find(|e| e["id"].as_i64() == Some(element_id));
        if let Some(player) = player_opt {
            let team_id = player["team"].as_i64().unwrap_or(0);
            let team_opt = teams.iter().find(|t| t["id"].as_i64() == Some(team_id));

            let web_name = player["web_name"].as_str().unwrap_or("Unknown");
            let team_short = team_opt
                .and_then(|t| t["short_name"].as_str())
                .unwrap_or("???");
            let event_points = player["event_points"].as_i64().unwrap_or(0);

            let mut display_name = web_name.to_string();
            if is_captain {
                display_name = format!("{} (C)", display_name);
            } else if is_vice {
                display_name = format!("{} (V)", display_name);
            }

            let player_info = (
                display_name,
                team_short.to_string(),
                event_points * multiplier,
            );

            if multiplier > 0 {
                starters.push(player_info);
            } else {
                bench.push(player_info);
            }
        }
    }

    let gw_points = entry_history["points"].as_i64().unwrap_or(0);
    let total_points = entry_history["total_points"].as_i64().unwrap_or(0);
    let overall_rank = entry_history["overall_rank"].as_i64().unwrap_or(0);
    let rank = entry_history["rank"].as_i64().unwrap_or(0);

    let mut description = String::new();

    if let Some(chip) = active_chip {
        description.push_str(&format!("**Active Chip:** {}\n\n", chip));
    }

    // Determine widths dynamically, but cap them to ensure table structure
    // Max width for name to fit in discord code block nicely
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
