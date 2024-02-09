// Desc: Help command
use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(slash_command, prefix_command, track_edits, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    println!("Sending help message");
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }
    let extra_text_at_bottom = "\
Type `!help command` for more info on a command.
You can edit your `!help` message to the bot and the bot will edit its response.\n
Source on GitHub: https://github.com/varunkamath/winnbot";

    let config = poise::samples::HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// Get source on GitHub
#[poise::command(slash_command, prefix_command, aliases("src"), category = "Utility")]
pub async fn source(ctx: Context<'_>) -> Result<(), Error> {
    // Build the embed with the github emoji (<:github:1198311705596399712>) and link to the source
    let embed = CreateEmbed::default()
        .title("Source")
        .description(
            "
            <:github:1198311705596399712> [varunkamath/winnbot](https://github.com/varunkamath/winnbot)",
        );
    let reply = poise::CreateReply::default().content("").embed(embed);
    ctx.send(reply).await?;
    Ok(())
}
