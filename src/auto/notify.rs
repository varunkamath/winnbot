use serenity::{
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::env;

pub async fn notify(msg: &Message, ctx: &Context, data: Vec<String>) {
    let mudae_id =
        env::var("MUDAE_ID").expect("Failed to get MUDAE_ID from the environment variables");
    if msg.author.id == mudae_id.parse::<u64>().unwrap() {
        if let Some(embed) = msg.embeds.first() {
            let name = embed.author.as_ref().unwrap().name.clone();
            let mut in_list = false;
            if data.contains(&name) {
                in_list = true;
                let index = data.iter().position(|r| r == &name).unwrap();
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
                            "{} is rank {} on the list\n[Message]({})",
                            name,
                            index + 1,
                            msg.link()
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            if !in_list {
                println!("{} is not in the list", name);
            }
        }
    }
}
