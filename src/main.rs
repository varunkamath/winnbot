pub mod auto;
pub mod commands;

extern crate sys_info;

use crate::auto::*;
use crate::commands::*;
use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

// use chrono::offset::Utc;
use serenity::builder::{CreateEmbed, CreateMessage};
// use serenity::gateway::ActivityData;
use serenity::model::id::{ChannelId, GuildId};

struct Handler {
    is_loop_running: AtomicBool,
    list_data: Vec<String>,
}

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
                content if content.starts_with("!rlrank") || content.starts_with("!r") => {
                    rank::rlrank(&msg, &ctx).await
                }
                "!sys" => log_system_load(&ctx).await,
                _ => unknown::unknown(&msg, &ctx).await,
            }
        } else {
            notify::notify(&msg, &ctx, self.list_data.clone()).await;
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");
        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    log_system_load(&ctx1).await;
                    tokio::time::sleep(Duration::from_secs(6000)).await;
                }
            });
            // let ctx2 = Arc::clone(&ctx);
            // tokio::spawn(async move {
            //     loop {
            //         set_activity_to_current_time(&ctx2);
            //         tokio::time::sleep(Duration::from_secs(60)).await;
            //     }
            // });
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn log_system_load(ctx: &Context) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();
    let embed = CreateEmbed::new()
        .title("System Resource Load")
        .field(
            "CPU Load Average",
            format!("{:.4}%", cpu_load.one * 10.0),
            false,
        )
        .field(
            "Memory Usage",
            format!(
                "Using {:.2}%\n{:.2} MB Free out of {:.2} MB",
                (mem_use.total as f32 - mem_use.free as f32) / mem_use.total as f32 * 100.0,
                mem_use.free as f32 / 1000.0,
                mem_use.total as f32 / 1000.0
            ),
            false,
        );
    let builder = CreateMessage::new().embed(embed);
    let message = ChannelId::new(822731141781389333)
        .send_message(&ctx, builder)
        .await;
    if let Err(why) = message {
        eprintln!("Error sending message: {why:?}");
    };
}

// fn set_activity_to_current_time(ctx: &Context) {
//     let current_time = Utc::now();
//     let formatted_time = current_time.to_rfc2822();

//     ctx.set_activity(Some(ActivityData::playing(formatted_time)));
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let list_data = include_str!("auto/data/data.txt")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
            list_data,
        })
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
