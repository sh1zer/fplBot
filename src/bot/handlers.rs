use serenity::{
    async_trait,
    all::{Interaction, Ready, Command, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    prelude::*,
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);

        let commands = vec![
            CreateCommand::new("hello").description("Say hello to the bot")
        ];

        match Command::set_global_commands(&ctx.http, commands).await {
            Ok(_) => info!("Successfully registered slash command"),
            Err(e) => info!("Failed to register slash command: {}", e),
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "hello" => "Hey!",
                _ => "Unknown command",
            };

            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                info!("Cannot respond to slash command: {}", why);
            }
        }
    }
}