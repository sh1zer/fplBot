use serenity::all::CommandOptionType;
use serenity::builder::{CreateCommand, CreateCommandOption};

/// Registers the track_fixture command with Discord
///
/// Creates the command definition for the `/track_fixture` slash command that allows
/// users to subscribe to fixture updates.
///
/// # Returns
/// * `CreateCommand` - Discord command definition ready for registration
///
/// # TODO
/// Implement the actual command handler and tracking functionality
pub fn register() -> CreateCommand {
    CreateCommand::new("track_fixture")
        .description("Get updates on given fixture")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "fixture_id", "The fixture ID")
                .required(true),
        )
}

// pub async fn run(_ctx: &Context, command: &CommandInteraction) -> Result<CreateInteractionResponse> {
//
// }
