// Desc: Count the number of messages in a channel
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, GetMessages};

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Count messages in the current channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("chc"),
    category = "Utility",
    help_text_fn = "count_help",
    on_error = "error_handler",
    owners_only
)]
pub async fn count(ctx: Context<'_>) -> Result<(), Error> {
    println!("Counting messages");
    ctx.defer_or_broadcast().await?;
    let mut count = 0;
    let time = std::time::Instant::now();
    let channel_id = ctx.channel_id();
    let builder = GetMessages::new().before(ctx.id()).limit(100);
    let mut messages = channel_id.messages(ctx.http(), builder).await?;
    while messages.len() > 0 {
        count += messages.len();
        let last_id = messages.last().unwrap().id;
        messages = channel_id
            .messages(ctx.http(), builder.before(last_id))
            .await
            .unwrap();
        println!("{}", count);
    }
    let reply = {
        let embed = CreateEmbed::new()
            .title("Message Count")
            .description(format!(
                "Counted {} messages in this channel. Time elapsed: {} seconds",
                count,
                time.elapsed().as_secs()
            ));
        poise::CreateReply::default().content("").embed(embed)
    };
    ctx.send(reply).await?;
    Ok(())
}

fn count_help() -> String {
    String::from("Count the number of messages in a channel")
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Error in command 'count': {}", error);
}
