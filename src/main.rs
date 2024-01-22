use async_rusqlite::Connection;
use dotenv::dotenv;
use rusqlite::params;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage, GetMessages},
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO: If the message is sent by this bot's user ID, ignore it
        if msg.content.starts_with("!") {
            if msg.content == "!help" || msg.content == "!h" {
                println!("Sending help message");
                let embed = CreateEmbed::new()
                .title("Help")
                .description("Commands:\n!help - Show this message\n!echo <message> - Echo a message\n!count - Count the number of messages in this channel\n!archive - Archive all messages in this channel\n\n[<:github:1198311705596399712> Source](https://github.com/varunkamath/winnbot)");
                // .footer(CreateEmbedFooter::new("by @telemtry"));
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                    println!("Error sending message: {:?}", why);
                }
            } else if msg.content.starts_with("!e") {
                println!("Echoing message");
                let content = msg.content[3..].trim();
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why);
                }
            } else if msg.content == "!count" {
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
            } else if msg.content == "!archive" {
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
            } else {
                println!("Unknown command");
                let embed = CreateEmbed::new()
                    .title("Unknown command")
                    .description("Use !help to see a list of commands");
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        let mudae_id =
            env::var("MUDAE_ID").expect("Failed to get MUDAE_ID from the environment variables");
        if msg.author.id == mudae_id.parse::<u64>().unwrap() {
            if let Some(embed) = msg.embeds.first() {
                let name = embed.author.as_ref().unwrap().name.clone();
                let data = include_str!("data.txt");
                let mut line_number = 0;
                let mut in_list = false;
                for line in data.lines() {
                    line_number += 1;
                    if line.to_lowercase() == name.to_lowercase() {
                        in_list = true;
                        let user_id = env::var("USER_ID")
                            .expect("Failed to get USER_ID from the environment variables");
                        let user = UserId::new(user_id.parse::<u64>().unwrap());
                        // Send message to user and link to message of the waifu
                        if let Err(why) = user
                            .create_dm_channel(&ctx.http)
                            .await
                            .unwrap()
                            .say(
                                &ctx.http,
                                format!(
                                    "{} is number {} in the list\n[Message]({})",
                                    name,
                                    line_number,
                                    msg.link()
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
                if !in_list {
                    println!("{} is not in the list", name);
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Failed to get DISCORD_TOKEN from the environment variables");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
