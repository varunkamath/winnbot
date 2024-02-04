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
    let mut content = msg.content.clone();
    if content.len() < 3 {
        println!("Not enough arguments");
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                "Not enough arguments. Usage: !r <username> <platform>",
            )
            .await;
        return;
    }
    if msg.content.starts_with("!rlrank") {
        content = msg.content[8..].trim().to_string();
    } else {
        content = msg.content[3..].trim().to_string();
    }
    println!("Content: {}", content);
    let mut args = content.split_whitespace();
    let num_args = args.clone().count();
    println!("Num args: {}", num_args);
    if num_args < 1 {
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
    let mut platform = "epic";
    if num_args == 1 {
        println!("Platform not specified");
        if username.to_lowercase() == "steam"
            || username.to_lowercase() == "epic"
            || username.to_lowercase() == "psn"
            || username.to_lowercase() == "xbox"
        {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Not enough arguments. Usage: !r <username> <platform>",
                )
                .await;
            return;
        }
    } else {
        platform = args.next().unwrap();
    }
    if username.to_lowercase() == "steam"
        || username.to_lowercase() == "epic"
        || username.to_lowercase() == "psn"
        || username.to_lowercase() == "xbox"
    {
        let temp = username;
        username = platform;
        platform = temp;
    }
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
        let args = PyTuple::new(py, &[username, platform]);
        py_resp = fun.call1(py, args).unwrap().to_string();
        Ok(())
    })
    .unwrap();
    let response = py_resp;
    println!("Response: {}", response);
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
    // Get MMR to next rank. MMR range for each rank is stored in ranks.txt, sample below:
    /*
    Supersonic Legend Division I, 1341 - 1527
    Grand Champion III Division I, 1282 - 1298
    Grand Champion III Division II, 1300 - 1313
    Grand Champion III Division III, 1318 - 1330
    Grand Champion III Division IV, 1337 - 1349
    Grand Champion II Division I, 1225 - 1238
    Grand Champion II Division II, 1239 - 1255
    Grand Champion II Division III, 1258 - 1270
    Grand Champion II Division IV, 1277 - 1286
    Grand Champion I Division I, 1175 - 1178
    Grand Champion I Division II, 1179 - 1197
    Grand Champion I Division III, 1198 - 1212
    Grand Champion I Division IV, 1217 - 1225
     */
    let mut mmr_to_next_rank = 0;
    let mut next_rank = "";
    let mut next_division = "";
    // Load ranks.txt
    // For all standard ranks, get MMR to next rank
    // for rank in std_ranks.clone() {
    //     let (name, rank, division, mmr, _) = rank;
    //     let rank_name = format!("{} {}", rank, division);
    //     for line in rank_mmrs.clone() {
    //         let rank_mmr: Vec<&str> = line.split(", ").collect();
    //         if rank_mmr[0] == rank_name {
    //             let mmr_range: Vec<&str> = rank_mmr[1].split(" - ").collect();
    //             let lower_mmr = mmr_range[0].parse::<u64>().unwrap();
    //             let upper_mmr = mmr_range[1].parse::<u64>().unwrap();
    //             mmr_to_next_rank = upper_mmr - mmr;
    //             next_rank = rank;
    //             next_division = division;
    //         }
    //     }
    // }
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
        format!("Highest Rank: {} {}\nMMR: {}", rank, division, mmr),
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
            "Ranked Duel 1v1" => include_str!("./1v1-ranks.txt"),
            "Ranked Doubles 2v2" => include_str!("./2v2-ranks.txt"),
            "Ranked Standard 3v3" => include_str!("./3v3-ranks.txt"),
            _ => include_str!("./2v2-ranks.txt"),
        };
        let rank_mmrs: Vec<&str> = rank_file.split("\n").collect();
        for line in rank_mmrs.clone() {
            let rank_mmr: Vec<&str> = line.split(", ").collect();
            if rank_mmr[0] == full_name {
                let mmr_range: Vec<&str> = rank_mmr[1].split(" - ").collect();
                let lower_mmr = mmr_range[0].parse::<u64>().unwrap();
                let upper_mmr = mmr_range[1].parse::<u64>().unwrap();
                println!("Upper MMR: {}", upper_mmr);
                mmr_to_next_rank = upper_mmr - mmr;
                // next_rank = rank;
                // next_division = division;
            }
        }
        embed = embed.field(
            *name,
            format!(
                "{} {} {}\nMMR: {}\nNext rank in {} MMR",
                rank_emoji, rank, division, mmr, mmr_to_next_rank
            ),
            false,
        )
    }
    embed = embed.color(0x00bfff);
    let builder = CreateMessage::new().content("").tts(false).embed(embed);
    let _ = msg.channel_id.send_message(&ctx.http, builder).await;
    return;
}
