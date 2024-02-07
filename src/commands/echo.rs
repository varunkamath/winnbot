// Desc: Echoes a message to the channel

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(
    // context_menu_command = "Echo a message",
    slash_command,
    prefix_command,
    aliases("c")
)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message to echo"] msg: String,
) -> Result<(), Error> {
    println!("Echoing message to channel");
    ctx.channel_id().say(ctx.http(), msg.clone()).await?;
    ctx.defer_ephemeral().await?;
    ctx.say(format!("Message echoed: \"{}\"", msg)).await?;
    Ok(())
}
