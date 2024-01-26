// Desc: Archive messages in a channel
use async_rusqlite::Connection;
use rusqlite::params;
use serenity::{
    builder::{CreateEmbed, CreateMessage, GetMessages},
    model::channel::Message,
    prelude::*,
};
use std::env;

pub async fn archive(msg: &Message, ctx: &Context) {
    println!("Archiving messages");
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if msg.author.id == user_id.parse::<u64>().unwrap() {
            let conn = Connection::open("discord.db").await.unwrap();
            let _ = conn
                .call(|conn| {
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS messages (
                    id TEXT PRIMARY KEY,
                    content TEXT NOT NULL,
                    author TEXT NOT NULL,
                    channel TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                )",
                        params![],
                    )
                })
                .await;
            let mut count = 0;
            let time = std::time::Instant::now();
            let channel_id = msg.channel_id;
            let builder = GetMessages::new().before(msg.id).limit(100);
            let mut messages = channel_id.messages(&ctx.http, builder).await.unwrap();
            while messages.len() > 0 {
                let last_id = messages.last().unwrap().id.clone();
                for message in messages {
                    let id = message.id.to_string();
                    let content = message.content;
                    let author = message.author.name;
                    let channel = message.channel_id.name(&ctx.http).await.unwrap();
                    let timestamp = message.timestamp.to_string();
                    let _ = conn.call(move |conn| {
                    conn.execute(
                        "INSERT INTO messages (id, content, author, channel, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![id, content, author, channel, timestamp],
                    )
                })
                .await;
                    count += 1;
                }
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
                        "Archived {} messages in this channel, time elapsed: {}",
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
