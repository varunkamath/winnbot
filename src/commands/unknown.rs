// Desc: Unknown command handling
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::channel::Message,
    prelude::*,
};

pub async fn unknown(msg: &Message, ctx: &Context) {
    println!("Unknown command");
    let embed = CreateEmbed::new()
        .title("Unknown command")
        .description("Use !help to see a list of commands");
    let builder = CreateMessage::new().content("").tts(false).embed(embed);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error sending message: {:?}", why);
    }
}
