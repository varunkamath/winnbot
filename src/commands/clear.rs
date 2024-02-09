// Desc: Clear messages from a channel
use poise::serenity_prelude as serenity;
use serenity::builder::GetMessages;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Clear messages in the current channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("c"),
    category = "Utility",
    help_text_fn = "clear_help",
    on_error = "error_handler",
    ephemeral,
    owners_only
)]
pub async fn clear(
    ctx: Context<'_>,
    #[description = "Number of messages to clear"] num_messages: Option<u8>,
) -> Result<(), Error> {
    println!("Clearing messages");
    ctx.defer_or_broadcast().await?;
    let num_messages = num_messages.unwrap_or(1);
    let channel_id = ctx.channel_id();
    let builder = GetMessages::new().before(ctx.id()).limit(num_messages);
    let messages = channel_id.messages(&ctx.http(), builder).await?;
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
    Ok(())
}

fn clear_help() -> String {
    String::from("Clear messages from a channel. Example usage: /clear 5")
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Error in command 'clear': {}", error);
}
