use serenity::{
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::env;

pub async fn notify(msg: &Message, ctx: &Context) {
    let mudae_id =
        env::var("MUDAE_ID").expect("Failed to get MUDAE_ID from the environment variables");
    if msg.author.id == mudae_id.parse::<u64>().unwrap() {
        if let Some(embed) = msg.embeds.first() {
            let name = embed.author.as_ref().unwrap().name.clone();
            let data = include_str!("data/data.txt");
            let mut line_number = 0;
            let mut in_list = false;
            for line in data.lines() {
                line_number += 1;
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
