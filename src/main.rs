// A simple Discord bot to help with the Winn Discord server.
use dotenv::dotenv;
use serenity::{
    async_trait,
    builder::GetMessages,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!") {
            if msg.content == "!e" {
                // Get message to echo
                let content = msg.content[3..].to_string();
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "!count" {
                println!("Counting messages");
                // If user's ID is user_id
                let user_id = env::var("USER_ID");
                if let Some(user_id) = user_id.ok() {
                    if msg.author.id == user_id.parse::<u64>().unwrap() {
                        // Count all the messages in the channel, and send the count and time taken
                        let mut count = 0;
                        let time = std::time::Instant::now();
                        // Get channel ID from the message
                        let channel_id = msg.channel_id;
                        // Use the GetMessages builder to get the messages (up to 100 at a time)
                        let builder = GetMessages::new().before(msg.id).limit(100);
                        let mut messages = channel_id.messages(&ctx.http, builder).await.unwrap();
                        // Loop until there are no more messages
                        while messages.len() > 0 {
                            // Add the number of messages to the count
                            count += messages.len();
                            // Get the ID of the last message
                            let last_id = messages.last().unwrap().id;
                            // Get the next 100 messages
                            messages = channel_id
                                .messages(&ctx.http, builder.before(last_id))
                                .await
                                .unwrap();
                            // Print current count
                            println!("{}", count);
                        }
                        // Send the count and time taken
                        if let Err(why) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!(
                                    "There are {} messages in this channel. It took {} seconds to count them.",
                                    count,
                                    time.elapsed().as_secs()
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
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
