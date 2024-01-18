// A simple Discord bot to help with the Winn Discord server.
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!") {
            let (command, content) = msg.content.split_at(2);
            if command == "!e" {
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        } else {
            let mudae_id = env::var("MUDAE_ID")
                .expect("Failed to get MUDAE_ID from the environment variables");
            if msg.author.id == mudae_id.parse::<u64>().unwrap() {
                if let Some(embed) = msg.embeds.first() {
                    let name = embed.author.as_ref().unwrap().name.clone();
                    let data = include_str!("data.txt");
                    let mut line_number = 0;
                    let mut in_list = false;
                    // If the name is in the list
                    for line in data.lines() {
                        line_number += 1;
                        // Fuzzy match entire line
                        if line.to_lowercase() == name.to_lowercase() {
                            in_list = true;
                            let user_id = env::var("USER_ID")
                                .expect("Failed to get USER_ID from the environment variables");
                            let user = UserId::new(user_id.parse::<u64>().unwrap());
                            if let Err(why) = user
                                .create_dm_channel(&ctx.http)
                                .await
                                .unwrap()
                                .say(
                                    &ctx.http,
                                    format!("{} is number {} in the list", name, line_number),
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
