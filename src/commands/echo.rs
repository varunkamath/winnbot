// Desc: Echoes a message to the channel

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Echo a provided message to the current channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("c"),
    category = "Utility",
    help_text_fn = "echo_help",
    on_error = "error_handler"
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

fn echo_help() -> String {
    String::from("Echo a message to the channel. Example usage: /echo Hello, world!")
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Error in command 'echo': {}", error);
}
