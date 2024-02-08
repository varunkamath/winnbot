// Desc: Prompt OpenAI GPT-4 Turbo to generate a response
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serenity::builder::CreateEmbed;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(
    slash_command,
    prefix_command,
    aliases("ai"),
    help_text_fn = "prompt_help",
    on_error = "error_handler",
    category = "AI",
    // context_menu_name = "Prompt GPT-4 Turbo"
)]
pub async fn prompt(
    ctx: Context<'_>,
    #[description = "Prompt to send to GPT-4 Turbo"] prompt: String,
) -> Result<(), Error> {
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if ctx.author().id != user_id.parse::<u64>().unwrap() {
            println!("User is not authorized!");
            let embed = CreateEmbed::new()
                .title("⚠️ Unauthorized")
                .description("You are not authorized to use this command");
            let reply = poise::CreateReply::default().content("").embed(embed);
            ctx.send(reply).await?;
            return Ok(());
        }
    }
    ctx.defer_or_broadcast().await?;
    println!("Sending prompt to GPT-4 Turbo");
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4-turbo-preview",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("Response: {:?}", response);
    let response = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response from GPT-4 Turbo")
        .to_string();
    let embed = CreateEmbed::new().description(response);
    let reply = poise::CreateReply::default().content("").embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    aliases("aiim"),
    help_text_fn = "imageprompt_help",
    on_error = "img_error_handler",
    category = "AI"
)]
pub async fn imageprompt(
    ctx: Context<'_>,
    #[description = "Prompt to send to DALL-E 3"] prompt: String,
) -> Result<(), Error> {
    let user_id = env::var("USER_ID");
    if let Some(user_id) = user_id.ok() {
        if ctx.author().id != user_id.parse::<u64>().unwrap() {
            println!("User is not authorized!");
            let embed = CreateEmbed::new()
                .title("⚠️ Unauthorized")
                .description("You are not authorized to use this command");
            let reply = poise::CreateReply::default().content("").embed(embed);
            ctx.send(reply).await?;
            return Ok(());
        }
    }
    ctx.defer_or_broadcast().await?;
    println!("Sending prompt to DALL-E 3");
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "dall-e-3",
            "prompt": prompt,
            "n": 1,
            "size": "1024x1024"
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("Response: {:?}", response);
    let response = response["data"][0]["url"]
        .as_str()
        .unwrap_or("No response from DALL-E 3")
        .to_string();
    let embed = CreateEmbed::new().image(response);
    let reply = poise::CreateReply::default().content("").embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

fn prompt_help() -> String {
    String::from("Prompt OpenAI GPT-4 Turbo to generate a response")
}

fn imageprompt_help() -> String {
    String::from("Prompt OpenAI DALL-E 3 to generate an image")
}

async fn error_handler(error: poise::FrameworkError<'_, crate::Data, Error>) {
    println!("Error in command 'prompt': {}", error);
}

async fn img_error_handler(error: poise::FrameworkError<'_, crate::Data, Error>) {
    println!("Error in command 'imageprompt': {}", error);
}
