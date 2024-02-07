use poise::serenity_prelude as serenity;
use serenity::{
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn notify(ctx: &Context, msg: &Message, list_data: Vec<String>) -> Result<(), Error> {
    if let Some(embed) = msg.embeds.first() {
        let name = embed.author.as_ref().unwrap().name.clone();
        let mut in_list = false;
        if list_data.contains(&name) {
            in_list = true;
            let index = list_data.iter().position(|r| r == &name).unwrap();
            let user_id =
                env::var("USER_ID").expect("Failed to get USER_ID from the environment variables");
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
    Ok(())
}
