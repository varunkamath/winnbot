// Desc: Clear messages from a channel
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateMessage, GetMessages};
use std::env;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    // context_menu_command = "Clear messages in a channel",
    slash_command,
    prefix_command,
    aliases("c"),
    category = "Utility",
    help_text_fn = "clear_help",
    on_error = "error_handler"
)]
pub async fn clear(
    ctx: Context<'_>,
    // Optional argument: number of messages to clear
    #[description = "Number of messages to clear"] num_messages: Option<u8>,
) -> Result<(), Error> {
    println!("Clearing messages");
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if ctx.author().id == user_id.parse::<u64>().unwrap() {
            let num_messages = num_messages.unwrap_or(1);
            let channel_id = ctx.channel_id();
            let builder = GetMessages::new().before(ctx.id()).limit(num_messages);
            let messages = channel_id.messages(&ctx.http(), builder).await?;
            // If slash command, do not delete the original message
            if ctx.prefix() != "/" {
                channel_id.delete_message(&ctx.http(), ctx.id()).await?;
            }
            channel_id
                .delete_messages(&ctx.http(), messages)
                .await
                .unwrap();
            println!("{} messages deleted", num_messages);
            ctx.defer_ephemeral().await?;
            ctx.say(format!("{} messages deleted", num_messages))
                .await?;
        } else {
            println!("User is not authorized!");
            let embed = CreateEmbed::new()
                .title("⚠️ Unauthorized")
                .description("You are not authorized to use this command");
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            if let Err(why) = ctx.channel_id().send_message(&ctx.http(), builder).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    Ok(())
}

fn clear_help() -> String {
    String::from("Clear messages from a channel. Example usage: /clear 5")
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Error in command 'clear': {}", error);
}
