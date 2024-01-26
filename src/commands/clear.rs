use serenity::{
    builder::{CreateEmbed, CreateMessage, GetMessages},
    model::channel::Message,
    prelude::*,
};
use std::env;

pub async fn clear(msg: &Message, ctx: &Context) {
    println!("Clearing messages");
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if msg.author.id == user_id.parse::<u64>().unwrap() {
            let mut content = msg.content[3..].trim().to_string();
            if msg.content.starts_with("!clear") {
                content = msg.content[6..].trim().to_string();
            }
            if content == "" {
                content = "1".to_string();
            }
            let num_messages = content.parse::<u8>().unwrap();
            let channel_id = msg.channel_id;
            let builder = GetMessages::new().before(msg.id).limit(num_messages);
            let messages = channel_id.messages(&ctx.http, builder).await.unwrap();
            channel_id
                .delete_messages(&ctx.http, messages)
                .await
                .unwrap();
            channel_id.delete_message(&ctx.http, msg.id).await.unwrap();
            println!("{} messages deleted", num_messages);
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
