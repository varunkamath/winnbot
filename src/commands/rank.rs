// Desc: Get Rocket League rank
use poise::serenity_prelude as serenity;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use serde_json::Value;
use serenity::builder::CreateEmbed;
use vsdbsled as sled;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::Data, Error>;

#[poise::command(
    slash_command,
    prefix_command,
    aliases("r"),
    track_edits,
    category = "Rocket League"
)]
pub async fn rlrank(
    ctx: Context<'_>,
    #[description = "Rocket League username"] username: Option<String>,
    #[description = "Platform (epic, steam, psn, xbox)"] platform: Option<String>,
) -> Result<(), Error> {
    println!("Getting Rocket League rank");
    ctx.defer_or_broadcast().await?;
    let db = sled::open("rl-usernames")?;
    let discord_id = ctx.author().id.to_string();
    let value = db.get(discord_id.clone())?;
    let username = match username {
        Some(username) => username,
        None => {
            if let Some(value) = value {
                let value = value.to_vec();
                let value = String::from_utf8(value).unwrap();
                let value: Vec<&str> = value.split("-").collect();
                value[0].to_string()
            } else {
                let embed = CreateEmbed::new()
                    .title("Rocket League Account")
                    .description("You have not registered your Rocket League account. Please use the `/rlregister` command to register your account or provide your username as an argument to this command.")
                    .color(0x00bfff);
                let reply = { poise::CreateReply::default().content("").embed(embed) };
                ctx.send(reply).await?;
                return Ok(());
            }
        }
    };
    // If platform is not provided, default to epic
    let platform = platform.unwrap_or("epic".to_string());
    println!("Username: {}", username);
    println!("Platform: {}", platform);
    let py_script = include_str!("./get_rank.py");
    let mut py_resp: String = "".to_string();
    Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code(py, py_script, "rank.py", "rank")
            .unwrap()
            .getattr("get_rank")
            .unwrap()
            .into();
        let args = PyTuple::new(py, &[username.clone(), platform]);
        py_resp = fun.call1(py, args).unwrap().to_string();
        Ok(())
    })
    .unwrap();
    let response = py_resp;
    // println!("Response: {}", response); // Uncomment to debug (some IPs are still blocked by Cloudflare)
    let json: Value = serde_json::from_str(&response).unwrap();
    let segments = json["data"]["segments"].as_array().unwrap();
    let mut ranks = vec![];
    for segment in segments {
        if segment["type"].as_str().unwrap() == "playlist" {
            let name = segment["metadata"]["name"].as_str().unwrap();
            let rank = segment["stats"]["tier"]["metadata"]["name"]
                .as_str()
                .unwrap();
            let division = segment["stats"]["division"]["metadata"]["name"]
                .as_str()
                .unwrap();
            let mmr = segment["stats"]["rating"]["value"].as_u64().unwrap();
            let rank_img_url = segment["stats"]["tier"]["metadata"]["iconUrl"]
                .as_str()
                .unwrap();
            ranks.push((name, rank, division, mmr, rank_img_url));
        }
    }
    let mut highest_mmr = 0;
    let mut highest_mmr_index = 0;

    let new_ranks = &ranks;
    for rank in new_ranks {
        println!("{:?}", rank);
    }
    let mut embed = CreateEmbed::new().title(format!("Rocket League Ranks: {}", username));
    let mut std_ranks = vec![];

    for rank in new_ranks {
        let (name, rank, division, mmr, rank_img_url) = rank;
        if *name == "Ranked Duel 1v1" {
            std_ranks.push((name, rank, division, mmr, rank_img_url));
        }
        if *name == "Ranked Doubles 2v2" {
            std_ranks.push((name, rank, division, mmr, rank_img_url));
        }
        if *name == "Ranked Standard 3v3" {
            std_ranks.push((name, rank, division, mmr, rank_img_url));
        }
    }
    let mut mmr_to_next_rank = 0;
    // let mut next_rank = "";
    // let mut next_division = "";
    for (i, rank) in std_ranks.iter().enumerate() {
        let (name, _, _, mmr, _) = rank;
        if **name == "Un-Ranked" {
            continue;
        }
        if **mmr > highest_mmr {
            highest_mmr = **mmr;
            highest_mmr_index = i;
        }
    }
    let (name, rank, division, mmr, rank_img_url) = std_ranks[highest_mmr_index];
    embed = embed.field(
        format!("Highest Ranked Standard Playlist: {}", name),
        format!(
            "Highest Rank: {} {}\nMMR: {}\n~~                                                                ~~",
            rank, division, mmr
        ),
        false,
    );
    embed = embed.thumbnail(*rank_img_url);
    for rank in std_ranks {
        let (name, rank, division, mmr, _) = rank;
        let rank_emoji = match *rank {
            "Bronze I" => "<:Bronze1:908453289945886760>",
            "Bronze II" => "<:Bronze2:908453289996197948>",
            "Bronze III" => "<:Bronze3:908453289421590549>",
            "Silver I" => "<:Silver1:908453289929089075>",
            "Silver II" => "<:Silver2:908453289769717791>",
            "Silver III" => "<:Silver3:908453289899728917>",
            "Gold I" => "<:Gold1:908453289799057428>",
            "Gold II" => "<:Gold2:908453289807470602>",
            "Gold III" => "<:Gold3:908453289857785876>",
            "Platinum I" => "<:Plat1:908453289643888721>",
            "Platinum II" => "<:Plat2:908453289878757396>",
            "Platinum III" => "<:Plat3:908453289572585543>",
            "Diamond I" => "<:Diamond1:908453289807446037>",
            "Diamond II" => "<:Diamond2:908453289903939584>",
            "Diamond III" => "<:Diamond3:908453289836814376>",
            "Champion I" => "<:Champ1:908453289820033104>",
            "Champion II" => "<:Champ2:908453289857781800>",
            "Champion III" => "<:Champ3:908453289945866250>",
            "Grand Champion I" => "<:GrandChamp1:908455642061238293>",
            "Grand Champion II" => "<:GrandChamp2:908455641838944276>",
            "Grand Champion III" => "<:GrandChamp3:908455642245759006>",
            "Supersonic Legend" => "<:SupersonicLegend:757171265768259664>",
            _ => "<:Unranked:908466188588310548>",
        };
        let full_name = format!("{} {}", rank, division);
        let rank_file = match *name {
            "Ranked Duel 1v1" => include_str!("./data/1v1-ranks.txt"),
            "Ranked Doubles 2v2" => include_str!("./data/2v2-ranks.txt"),
            "Ranked Standard 3v3" => include_str!("./data/3v3-ranks.txt"),
            _ => include_str!("./data/2v2-ranks.txt"),
        };
        let rank_mmrs: Vec<&str> = rank_file.split("\n").collect();
        for line in rank_mmrs.clone() {
            let rank_mmr: Vec<&str> = line.split(", ").collect();
            if *name == "Supersonic Legend" {
                // next_rank = "N/A";
                // next_division = "N/A";
                mmr_to_next_rank = 0;
                break;
            }
            if rank_mmr[0] == full_name {
                let next_rank_mmr =
                    rank_mmrs[rank_mmrs.iter().position(|&x| x == line).unwrap() + 1];
                let next_rank_mmr: Vec<&str> = next_rank_mmr.split(", ").collect();
                println!("{:?}", next_rank_mmr);
                // next_rank = next_rank_mmr[0].split(" ").collect::<Vec<&str>>()[1];
                // next_division = next_rank_mmr[0].split(" ").collect::<Vec<&str>>()[2];
                mmr_to_next_rank = next_rank_mmr[1].split(" - ").collect::<Vec<&str>>()[0]
                    .parse::<u64>()
                    .unwrap()
                    - *mmr;
                break;
            }
        }
        embed = embed.field(
            format!("__{}__", *name),
            format!(
                "{} {} {}\nMMR: {}\n**Next division in {} MMR**",
                rank_emoji, rank, division, mmr, mmr_to_next_rank
            ),
            false,
        )
    }
    embed = embed.color(0x00bfff);
    let reply = { poise::CreateReply::default().content("").embed(embed) };
    ctx.send(reply).await?;
    println!("Sent message");
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    aliases("rar"),
    track_edits,
    category = "Rocket League"
)]
pub async fn rlregister(
    ctx: Context<'_>,
    #[description = "Rocket League username"] username: String,
    #[description = "Platform (epic, steam, psn, xbox)"] platform: Option<String>,
) -> Result<(), Error> {
    let db = sled::open("rl-usernames")?;
    let discord_id = ctx.author().id.to_string();
    let value = db.get(discord_id.clone())?;
    if let Some(value) = value {
        let value = value.to_vec();
        let value = String::from_utf8(value).unwrap();
        let value: Vec<&str> = value.split("-").collect();
        let username = value[0];
        let platform = value[1];
        let embed = CreateEmbed::new()
            .title("Rocket League Account Already Registered")
            .field("Username", username, true)
            .field("Platform", platform, true)
            .color(0x00bfff);
        let reply = { poise::CreateReply::default().content("").embed(embed) };
        ctx.send(reply).await?;
        return Ok(());
    }
    let platform = platform.unwrap_or("epic".to_string());
    let discord_id = ctx.author().id.to_string();
    let key = discord_id.clone();
    let value = format!("{}-{}", username, platform);
    let value = sled::IVec::from(value.as_bytes());
    db.insert(key, value)?;
    let embed = CreateEmbed::new()
        .title("New Rocket League Account Registered")
        .field("Username", username, true)
        .field("Platform", platform, true)
        .color(0x00bfff);
    let reply = { poise::CreateReply::default().content("").embed(embed) };
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    aliases("rac"),
    track_edits,
    category = "Rocket League"
)]
pub async fn rlaccount(ctx: Context<'_>) -> Result<(), Error> {
    let db = sled::open("rl-usernames")?;
    let discord_id = ctx.author().id.to_string();
    let value = db.get(discord_id)?;
    if let Some(value) = value {
        let value = value.to_vec();
        let value = String::from_utf8(value).unwrap();
        let value: Vec<&str> = value.split("-").collect();
        let username = value[0];
        let platform = value[1];
        let embed = CreateEmbed::new()
            .title("Rocket League Account")
            .field("Username", username, true)
            .field("Platform", platform, true)
            .color(0x00bfff);
        let reply = { poise::CreateReply::default().content("").embed(embed) };
        ctx.send(reply).await?;
    } else {
        let embed = CreateEmbed::new()
            .title("Rocket League Account")
            .description("You have not registered your Rocket League account")
            .color(0x00bfff);
        let reply = { poise::CreateReply::default().content("").embed(embed) };
        ctx.send(reply).await?;
    }
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    aliases("rad"),
    track_edits,
    category = "Rocket League"
)]
pub async fn rldelete(ctx: Context<'_>) -> Result<(), Error> {
    let db = sled::open("rl-usernames")?;
    let discord_id = ctx.author().id.to_string();
    let value = db.get(discord_id.clone())?;
    if let Some(value) = value {
        db.remove(discord_id)?;
        let value = value.to_vec();
        let value = String::from_utf8(value).unwrap();
        let value: Vec<&str> = value.split("-").collect();
        let username = value[0];
        let platform = value[1];
        let embed = CreateEmbed::new()
            .title("Rocket League Account Deleted")
            .field("Username", username, true)
            .field("Platform", platform, true)
            .color(0x00bfff);
        let reply = { poise::CreateReply::default().content("").embed(embed) };
        ctx.send(reply).await?;
    } else {
        let embed = CreateEmbed::new()
            .title("Rocket League Account")
            .description("You have not registered your Rocket League account")
            .color(0x00bfff);
        let reply = { poise::CreateReply::default().content("").embed(embed) };
        ctx.send(reply).await?;
    }
    Ok(())
}
