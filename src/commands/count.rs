use serenity::{
    builder::{CreateEmbed, CreateMessage, GetMessages},
    model::channel::Message,
    prelude::*,
};
use std::env;

pub async fn count(msg: &Message, ctx: &Context) {
    println!("Counting messages");
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if msg.author.id == user_id.parse::<u64>().unwrap() {
            let mut count = 0;
            let time = std::time::Instant::now();
            let channel_id = msg.channel_id;
            let builder = GetMessages::new().before(msg.id).limit(100);
            let mut messages = channel_id.messages(&ctx.http, builder).await.unwrap();
            while messages.len() > 0 {
                count += messages.len();
                let last_id = messages.last().unwrap().id;
                messages = channel_id
                    .messages(&ctx.http, builder.before(last_id))
                    .await
                    .unwrap();
                println!("{}", count);
            }
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Counted {} messages in this channel, time elapsed: {}",
                        count,
                        time.elapsed().as_secs_f32()
                    ),
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        } else {
            println!("User is not authorized!");
            let embed = CreateEmbed::new()
                .title("⚠️ Unauthorized")
                .description("You are not authorized to use this command");
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
