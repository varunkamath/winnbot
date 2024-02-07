// Desc: Count the number of messages in a channel
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateMessage, GetMessages};
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(slash_command, prefix_command, aliases("chc"))]
pub async fn count(ctx: Context<'_>) -> Result<(), Error> {
    println!("Counting messages");
    let user_id = env::var("USER_ID");
    ctx.defer_or_broadcast().await?;
    if let Some(user_id) = user_id.ok() {
        if ctx.author().id == user_id.parse::<u64>().unwrap() {
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
            // let embed = CreateEmbed::new()
            //     .title("Message Count")
            //     .description(format!(
            //         "Counted {} messages in this channel. Time elapsed: {} seconds",
            //         count,
            //         time.elapsed().as_secs()
            //     ));
            // let builder = CreateMessage::new().content("").tts(false).embed(embed);
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
        } else {
            println!("User is not authorized!");
            let embed = CreateEmbed::new()
                .title("⚠️ Unauthorized")
                .description("You are not authorized to use this command");
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            if let Err(why) = ctx.channel_id().send_message(ctx.http(), builder).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    Ok(())
}
