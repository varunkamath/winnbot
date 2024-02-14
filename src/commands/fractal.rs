// Desc: Generate a fractal image with the given parameters.
// use fractal_renderer::render;
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateEmbed, CreateMessage};

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    prefix_command,
    aliases("frac"),
    category = "Rendering",
    help_text_fn = "fractal_help",
    on_error = "error_handler"
)]
pub async fn fractal(ctx: Context<'_>) -> Result<(), Error> {
    // let fractal = render();
    Ok(())
}

fn fractal_help() -> String {
    String::from("Generates a fractal image with the given parameters")
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Error in command 'fractal': {}", error);
}
