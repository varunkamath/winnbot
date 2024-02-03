// Desc: Get Rocket League rank
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use serde_json::Value;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::channel::Message,
    prelude::*,
};

pub async fn rlrank(msg: &Message, ctx: &Context) {
    println!("Getting Rocket League rank");
    let mut content = msg.content[3..].trim();
    if msg.content.starts_with("!rlrank") {
        content = msg.content[8..].trim();
    }
    let mut args = content.split_whitespace();
    if args.clone().count() < 2 {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                "Not enough arguments. Usage: !r <username> <platform>",
            )
            .await;
        return;
    }
    let mut username = args.next().unwrap();
    let mut platform = args.next().unwrap();
    // If platform is the first argument, swap the arguments
    if username.to_lowercase() == "steam"
        || username.to_lowercase() == "epic"
        || username.to_lowercase() == "psn"
        || username.to_lowercase() == "xbox"
    {
        // Swap the arguments
        let temp = username;
        username = platform;
        platform = temp;
    }
    println!("Username: {}", username);
    println!("Platform: {}", platform);
    let py_script = include_str!("./get_rank.py");
    Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code(py, py_script, "rank.py", "rank")
            .unwrap()
            .getattr("get_rank")
            .unwrap()
            .into();
        let args = PyTuple::new(py, &[username, platform]);
        fun.call1(py, args).unwrap();
        Ok(())
    })
    .unwrap();
    let response = std::fs::read_to_string("./response.json").unwrap();
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
    for (i, rank) in ranks.iter().enumerate() {
        let (name, _, _, mmr, _) = rank;
        if *name == "Un-Ranked" {
            continue;
        }
        if mmr > &highest_mmr {
            highest_mmr = *mmr;
            highest_mmr_index = i;
        }
    }

    let new_ranks = &ranks;
    for rank in new_ranks {
        println!("{:?}", rank);
    }
    let mut embed = CreateEmbed::new().title(format!("Rocket League Ranks: {}", username));
    let (name, rank, division, mmr, rank_img_url) = ranks[highest_mmr_index];
    embed = embed.field(
        format!("Highest Ranked Playlist: {}", name),
        format!(
            "Highest Rank: {}\nDivision: {}\nMMR: {}",
            rank, division, mmr
        ),
        false,
    );
    embed = embed.thumbnail(rank_img_url);
    for rank in ranks {
        let (name, rank, division, mmr, _) = rank;
        embed = embed.field(
            name,
            format!("Rank: {}\nDivision: {}\nMMR: {}", rank, division, mmr),
            true,
        )
        // .image(rank_img_url);
    }
    embed = embed.color(0x0000ff);
    let builder = CreateMessage::new().content("").tts(false).embed(embed);
    let _ = msg.channel_id.send_message(&ctx.http, builder).await;
    std::fs::remove_file("response.json").unwrap();
    return;
}
