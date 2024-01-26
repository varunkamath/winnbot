// Desc: Help command
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::channel::Message,
    prelude::*,
};

pub async fn help(msg: &Message, ctx: &Context) {
    println!("Sending help message");
    let embed = CreateEmbed::new()
    .title("Help")
    .description("Commands:\n!help - Show this message\n!echo <message> - Echo a message\n!count - Count the number of messages in this channel\n!archive - Archive all messages in this channel\n!puzzle - Solve a random chess puzzle (UCI notation only)\n\n[<:github:1198311705596399712> Source](https://github.com/varunkamath/winnbot)");
    // .footer(CreateEmbedFooter::new("by @telemtry"));
    let builder = CreateMessage::new().content("").tts(false).embed(embed);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error sending message: {:?}", why);
    }
}
