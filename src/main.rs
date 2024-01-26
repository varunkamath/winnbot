pub mod auto;
pub mod commands;

use crate::auto::*;
use crate::commands::*;
use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO: If the message is sent by this bot's user ID, ignore it
        if msg.content.starts_with("!") {
            match msg.content.as_str() {
                "!help" | "!h" => help::help(&msg, &ctx).await,
                content if content.starts_with("!e") => echo::echo(&msg, &ctx).await,
                "!count" => count::count(&msg, &ctx).await,
                "!archive" => archive::archive(&msg, &ctx).await,
                content if content.starts_with("!clear") || content.starts_with("!c") => {
                    clear::clear(&msg, &ctx).await
                }
                "!puzzle" | "!p" | "!solution" | "!sol" => puzzle::puzzle(&msg, &ctx).await,
                _ => unknown::unknown(&msg, &ctx).await,
            }
        } else {
            notify::notify(&msg, &ctx).await;
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
