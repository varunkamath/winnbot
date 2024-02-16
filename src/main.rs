pub mod auto;
pub mod commands;

extern crate sys_info;

use crate::auto::*;
use crate::commands::*;
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::id::ChannelId;
use std::env;
use std::sync::Arc;
use std::time::Duration;

struct Data {
    list_data: Vec<String>,
    mudae_id: u64,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn log_system_load(ctx: &poise::serenity_prelude::Context) -> Result<(), Error> {
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
    let _ = ChannelId::new(822731141781389333)
        .send_message(&ctx, builder)
        .await?;
    println!("Resource use logged, all systems nominal");
    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            let ctx1 = Arc::new(ctx.clone());
            tokio::spawn(async move {
                loop {
                    log_system_load(&ctx1).await.unwrap();
                    tokio::time::sleep(Duration::from_secs(6000)).await;
                }
            });
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.id == data.mudae_id {
                notify::notify(ctx, new_message, data.list_data.clone()).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[poise::command(prefix_command)]
async fn slash_register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let mudae_id = env::var("MUDAE_ID")
        .expect("Failed to get MUDAE_ID from the environment variables")
        .parse::<u64>()
        .unwrap();
    let list_data = include_str!("auto/data/data.txt")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                clear::clear(),
                count::count(),
                echo::echo(),
                help::help(),
                help::source(),
                puzzle::puzzle(),
                puzzle::solution(),
                rank::rlrank(),
                rank::rlregister(),
                rank::rlaccount(),
                rank::rldelete(),
                prompt::prompt(),
                prompt::imageprompt(),
                slash_register(),
                // unknown::unknown,
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![
                    poise::Prefix::Literal("🐱 "),
                    poise::Prefix::Literal("🐱"),
                    poise::Prefix::Literal("<:varunhappy:823224950738911242> "),
                    poise::Prefix::Literal("<:varunhappy:823224950738911242>"),
                    poise::Prefix::Regex(
                        "(yo|hey) (kitten|winn|winny),? can you (please |pwease )?"
                            .parse()
                            .unwrap(),
                    ),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(serenity::ActivityData::watching("for /help")));
                Ok(Data {
                    list_data,
                    mudae_id,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
