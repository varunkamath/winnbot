// Desc: Prompt OpenAI GPT-4 Turbo to generate a response
use poise::serenity_prelude as serenity;
use reqwest::Client;
use serenity::builder::CreateEmbed;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(slash_command, prefix_command, aliases("ai"))]
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

#[poise::command(slash_command, prefix_command, aliases("aiim"))]
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
    /* Sample request
      https://api.openai.com/v1/images/generations \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
      "model": "dall-e-3",
      "prompt": "a white siamese cat",
      "n": 1,
      "size": "1024x1024"
    }'
       */
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
    /* Sample response:
    Response: Object {"created": Number(1707416476), "data": Array [Object {"revised_prompt": String("An image illustrating a small monkey, its fur a mix of browns and blacks. It is hanging lively from a thick branch of a tropical rainforest tree. The monkey's small round eyes shimmer with curiosity as it peers into the distance. Its long, slender tail, shaped like a question mark, adds an extra layer of balance as it swings in the misty rainforest environment. The scene is also adorned with lush green vegetation and vividly colored tropical flowers."), "url": String("https://oaidalleapiprodscus.blob.core.windows.net/private/org-nVewRpVr4F7pd8CAqsQqVOVA/user-VUuMtxs6n1lqvJ0irminKXEk/img-ZCZLeuFy1Og9AuxVrUpTbClR.png?st=2024-02-08T17%3A21%3A16Z&se=2024-02-08T19%3A21%3A16Z&sp=r&sv=2021-08-06&sr=b&rscd=inline&rsct=image/png&skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&sktid=a48cca56-e6da-484e-a814-9c849652bcb3&skt=2024-02-08T03%3A22%3A10Z&ske=2024-02-09T03%3A22%3A10Z&sks=b&skv=2021-08-06&sig=rEWweoaNnDpMmRhuGCu/g5usScPbdpnNmensv/dlNbo%3D")}]} */
    let response = response["data"][0]["url"]
        .as_str()
        .unwrap_or("No response from DALL-E 3")
        .to_string();
    let embed = CreateEmbed::new().image(response);
    let reply = poise::CreateReply::default().content("").embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
