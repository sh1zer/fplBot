use std::borrow::Cow;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage, ButtonStyle};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed, CreateButton};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use crate::fpl::models::league::{LeagueStandings, StandingsManager};
use anyhow::{Result, anyhow};

pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
    let league_id = extract_league_id(&command.data.options())?;
    let standings = LeagueStandings::fetch(league_id).await?;
    
    let page = 0;
    let embed = build_standings_embed(&standings, page);
    let buttons = build_navigation_buttons(page, &standings);

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .button(buttons.prev)
            .button(buttons.next)
            .button(buttons.refresh)
    ))
}

fn extract_league_id(options: &[ResolvedOption]) -> Result<i32> {
    match options.first() {
        Some(ResolvedOption {
            value: ResolvedValue::Integer(id), ..
        }) => Ok(*id as i32),
        _ => Err(anyhow!("Please provide a valid league ID")),
    }
}

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

pub struct NavigationButtons {
    pub prev: CreateButton,
    pub next: CreateButton,
    pub refresh: CreateButton,
}

pub fn build_navigation_buttons(page: usize, standings: &LeagueStandings) -> NavigationButtons {
    let per_page = 25;
    let total_managers = standings.standings.managers.len();
    let api_has_next = standings.standings.has_next;
    let current_api_page = standings.standings.page;
    let total_pages_current = (total_managers + per_page - 1) / per_page;
    let has_prev = page > 0;
    let has_next = page + 1 < total_pages_current || api_has_next;
    
    NavigationButtons {
        prev: CreateButton::new(format!("standings_prev_{}_{}", page, current_api_page))
            .label("⬅️ Previous")
            .style(ButtonStyle::Secondary)
            .disabled(!has_prev),
        next: CreateButton::new(format!("standings_next_{}_{}", page, current_api_page))
            .label("Next ➡️")
            .style(ButtonStyle::Secondary)
            .disabled(!has_next),
        refresh: CreateButton::new(format!("standings_refresh_{}_{}", page, current_api_page))
            .label("🔄 Refresh")
            .style(ButtonStyle::Primary),
    }
}

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

