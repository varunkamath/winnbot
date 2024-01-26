// Desc: Echoes a message to the channel
use serenity::{model::channel::Message, prelude::*};

pub async fn echo(msg: &Message, ctx: &Context) {
    println!("Echoing message to channel");
    let content = msg.content[3..].trim();
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {:?}", why);
    }
}
