use async_rusqlite::Connection;
use dotenvy::dotenv;
use image::{self, DynamicImage, GenericImageView};
use pgnparse::parser::*;
use rand::seq::SliceRandom;
use reqwest::{get, Response};
use rusqlite::params;
use serde_json::Value;
use serenity::{
    async_trait,
    builder::{CreateAttachment, CreateEmbed, CreateMessage, GetMessages},
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use shakmaty::*;
use std::{env, fmt::Debug};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO: If the message is sent by this bot's user ID, ignore it
        if msg.content.starts_with("!") {
            if msg.content == "!help" || msg.content == "!h" {
                println!("Sending help message");
                let embed = CreateEmbed::new()
                .title("Help")
                .description("Commands:\n!help - Show this message\n!echo <message> - Echo a message\n!count - Count the number of messages in this channel\n!archive - Archive all messages in this channel\n\n[<:github:1198311705596399712> Source](https://github.com/varunkamath/winnbot)");
                // .footer(CreateEmbedFooter::new("by @telemtry"));
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                    println!("Error sending message: {:?}", why);
                }
            } else if msg.content.starts_with("!e") {
                println!("Echoing message to channel");
                let content = msg.content[3..].trim();
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why);
                }
            } else if msg.content == "!count" {
                println!("Counting messages");
                let user_id = env::var("USER_ID");
                if let Some(user_id) = user_id.ok() {
                    if msg.author.id == user_id.parse::<u64>().unwrap() {
                        let mut count = 0;
                        let time = std::time::Instant::now();
                        let channel_id = msg.channel_id;
                        let builder = GetMessages::new().before(msg.id).limit(100);
                        let mut messages = channel_id.messages(&ctx.http, builder).await.unwrap();
                        while messages.len() > 0 {
                            count += messages.len();
                            let last_id = messages.last().unwrap().id;
                            messages = channel_id
                                .messages(&ctx.http, builder.before(last_id))
                                .await
                                .unwrap();
                            println!("{}", count);
                        }
                        if let Err(why) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!(
                                    "Counted {} messages in this channel, time elapsed: {}",
                                    count,
                                    time.elapsed().as_secs_f32()
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    } else {
                        println!("User is not authorized!");
                        let embed = CreateEmbed::new()
                            .title("⚠️ Unauthorized")
                            .description("You are not authorized to use this command");
                        let builder = CreateMessage::new().content("").tts(false).embed(embed);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            } else if msg.content == "!archive" {
                println!("Archiving messages");
                let user_id = env::var("USER_ID");
                if let Some(user_id) = user_id.ok() {
                    if msg.author.id == user_id.parse::<u64>().unwrap() {
                        let conn = Connection::open("discord.db").await.unwrap();
                        let _ = conn
                            .call(|conn| {
                                conn.execute(
                                    "CREATE TABLE IF NOT EXISTS messages (
                                id TEXT PRIMARY KEY,
                                content TEXT NOT NULL,
                                author TEXT NOT NULL,
                                channel TEXT NOT NULL,
                                timestamp TEXT NOT NULL
                            )",
                                    params![],
                                )
                            })
                            .await;
                        let mut count = 0;
                        let time = std::time::Instant::now();
                        let channel_id = msg.channel_id;
                        let builder = GetMessages::new().before(msg.id).limit(100);
                        let mut messages = channel_id.messages(&ctx.http, builder).await.unwrap();
                        while messages.len() > 0 {
                            let last_id = messages.last().unwrap().id.clone();
                            for message in messages {
                                let id = message.id.to_string();
                                let content = message.content;
                                let author = message.author.name;
                                let channel = message.channel_id.name(&ctx.http).await.unwrap();
                                let timestamp = message.timestamp.to_string();
                                let _ = conn.call(move |conn| {
                                conn.execute(
                                    "INSERT INTO messages (id, content, author, channel, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    params![id, content, author, channel, timestamp],
                                )
                            })
                            .await;
                                count += 1;
                            }
                            messages = channel_id
                                .messages(&ctx.http, builder.before(last_id))
                                .await
                                .unwrap();
                            println!("{}", count);
                        }
                        if let Err(why) = msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!(
                                    "Archived {} messages in this channel, time elapsed: {}",
                                    count,
                                    time.elapsed().as_secs_f32()
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    } else {
                        println!("User is not authorized!");
                        let embed = CreateEmbed::new()
                            .title("⚠️ Unauthorized")
                            .description("You are not authorized to use this command");
                        let builder = CreateMessage::new().content("").tts(false).embed(embed);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            } else if msg.content.starts_with("!clear") || msg.content.starts_with("!c") {
                println!("Clearing messages");
                let user_id = env::var("USER_ID");
                if let Some(user_id) = user_id.ok() {
                    if msg.author.id == user_id.parse::<u64>().unwrap() {
                        let mut content = msg.content[3..].trim().to_string();
                        if msg.content.starts_with("!clear") {
                            content = msg.content[6..].trim().to_string();
                        }
                        if content == "" {
                            content = "1".to_string();
                        }
                        let num_messages = content.parse::<u8>().unwrap();
                        let channel_id = msg.channel_id;
                        let builder = GetMessages::new().before(msg.id).limit(num_messages);
                        let messages = channel_id.messages(&ctx.http, builder).await.unwrap();
                        channel_id
                            .delete_messages(&ctx.http, messages)
                            .await
                            .unwrap();
                        channel_id.delete_message(&ctx.http, msg.id).await.unwrap();
                        println!("{} messages deleted", num_messages);
                    } else {
                        println!("User is not authorized!");
                        let embed = CreateEmbed::new()
                            .title("⚠️ Unauthorized")
                            .description("You are not authorized to use this command");
                        let builder = CreateMessage::new().content("").tts(false).embed(embed);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            } else if msg.content == "!puzzle" || msg.content == "!p" {
                let file_path = std::path::Path::new("./puzzle.png");
                if file_path.exists() {
                    println!("Puzzle already in progress");
                    let embed = CreateEmbed::new()
                        .title("Puzzle already in progress")
                        .description("Solve the puzzle or wait for the timeout (30s)");
                    let builder = CreateMessage::new().content("").tts(false).embed(embed);
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
                let file_contents = include_str!("../puzzles.csv");
                let lines = file_contents.lines().collect::<Vec<&str>>();
                let line = lines.choose(&mut rand::thread_rng()).unwrap();
                println!("Sending puzzle {}", line);
                let url = format!("https://lichess.org/api/puzzle/{}", line);
                let response = reqwest::get(&url).await.unwrap();
                let json: Value = response.json().await.unwrap();
                let pgn = json["game"]["pgn"].as_str().unwrap();
                let pgn = parse_pgn_to_rust_struct(pgn);
                let fen = pgn.moves.last().unwrap().fen_after.clone();
                let board_fen = fen.split(' ').collect::<Vec<&str>>()[0];
                let mut whose_turn = fen.split(' ').collect::<Vec<&str>>()[1];
                let mut board_turn = shakmaty::Color::White;
                println!("Default turn: {}", board_turn);
                if whose_turn == "w" {
                    whose_turn = "White";
                    board_turn = shakmaty::Color::White;
                } else {
                    whose_turn = "Black";
                    board_turn = shakmaty::Color::Black;
                }
                println!("{}", fen);
                let solution = json["puzzle"]["solution"].as_array().unwrap();
                let solution = solution
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect::<Vec<&str>>();
                let solution = solution.join(" ");
                let url = format!("https://fen2image.chessvision.ai/{}", fen);
                let response = reqwest::get(&url).await.unwrap();
                let image = response.bytes().await.unwrap();
                let image = image::load_from_memory(&image).unwrap();
                let (width, _) = image.dimensions();
                let image = image.crop_imm(0, 0, width, width);
                image.save("puzzle.png").unwrap();
                let attachment = CreateAttachment::path("./puzzle.png").await;
                let embed = CreateEmbed::new()
                    .title("Puzzle")
                    // .description(format!("{} to move.\nSolution: {}", whose_turn, solution))
                    .description(format!("{} to move.", whose_turn))
                    .attachment("puzzle.png");
                // Image as attachment using ChannelId::send_files
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                let _ = msg
                    .channel_id
                    .send_files(&ctx.http, attachment, builder)
                    .await
                    .unwrap();
                let board = shakmaty::Board::from_ascii_board_fen(board_fen.as_bytes()).unwrap();
                // Print the board
                println!("{:?}", board.occupied());
                // Convert to Setup
                let mut setup = Setup::empty();
                setup.board = board;
                setup.turn = board_turn;
                let mode = shakmaty::CastlingMode::Standard;
                let pos = Chess::from_setup(setup, mode).unwrap();
                let legals = pos.legal_moves();
                // Convert legals to UCI (e4e5, etc.). Legals looks like this: [Normal { role: Pawn, from: C4, capture: None, to: C3, promotion: None }, Normal { role: Pawn, from: H4, capture: None, to: H3, promotion: None }, Normal { role: Bishop, from: B5, capture: None, to: A4, promotion: None }, Normal { role: Bishop, from: B5, capture: None, to: C6, promotion: None }, Normal { role: Bishop, from: B5, capture: None, to: D7, promotion: None }, Normal { role: Bishop, from: B5, capture: None, to: E8, promotion: None }, Normal { role: King, from: H7, capture: None, to: H6, promotion: None }, Normal { role: King, from: H7, capture: None, to: G8, promotion: None }, Normal { role: King, from: H7, capture: None, to: H8, promotion: None }]
                let mut uci_legals = Vec::new();
                for legal in legals {
                    uci_legals.push(legal.to_string());
                }
                // Now uci_legals looks like this: ["d4xc5", "a2-a3", "b2-b3", "g2-g3", "h2-h3", "e3-e4", "a2-a4", "b2-b4", "g2-g4", "h2-h4", "Nf3-d2", "Nf3-h4", "Nf3-e5", "Nf3-g5", "Bc1-d2", "Bc2-b1", "Bc2-d1", "Bc2-b3", "Bc2-a4", "Ra1-b1", "Re1-d1", "Re1-f1", "Re1-e2", "Qd3-d1", "Qd3-f1", "Qd3-d2", "Qd3-e2", "Qd3-a3", "Qd3-b3", "Qd3-c3", "Qd3-c4", "Qd3-e4", "Qd3-b5", "Qd3-f5", "Qd3-a6", "Qd3-g6", "Qd3xh7", "Kg1-f1", "Kg1-h1"]
                // Convert to proper UCI (e2e4, etc.) by removing the piece and capture information
                for uci_legal in &mut uci_legals {
                    *uci_legal = uci_legal.replace("x", "");
                    *uci_legal = uci_legal.replace("-", "");
                    *uci_legal = uci_legal.replace("N", "");
                    *uci_legal = uci_legal.replace("B", "");
                    *uci_legal = uci_legal.replace("R", "");
                    *uci_legal = uci_legal.replace("Q", "");
                    *uci_legal = uci_legal.replace("K", "");
                }
                // Print the legals
                println!("Legal moves: {:?}", uci_legals);
                // Start listening for a response from the user with a timeout of 30 seconds
                let mut correct = false;
                let mut timeout = false;
                let solution = solution;
                // Get time now
                let time = std::time::Instant::now();
                // Get next message from user who sent the command
                let mut messages = msg
                    .channel_id
                    .messages(&ctx.http, GetMessages::new().limit(1))
                    .await
                    .unwrap();
                let mut message = messages.pop().unwrap();
                while correct == false {
                    while message.author.id != msg.author.id {
                        messages = msg
                            .channel_id
                            .messages(&ctx.http, GetMessages::new().limit(1))
                            .await
                            .unwrap();
                        message = messages.pop().unwrap();
                    }
                    if message.author.bot == true {
                        continue;
                    }
                    println!("{}", time.elapsed().as_secs_f32());
                    if time.elapsed().as_secs_f32() > 30.0 {
                        if let Err(why) = message.react(&ctx.http, '⏰').await {
                            println!("Error reacting to message: {:?}", why);
                        }
                        let embed = CreateEmbed::new()
                            .title("Time's up!")
                            .description(format!("Solution: {}", solution));
                        let builder = CreateMessage::new().content("").tts(false).embed(embed);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                        timeout = true;
                        break;
                    }
                    if !uci_legals.contains(&message.content.to_lowercase()) {
                        if let Err(why) = message.react(&ctx.http, '❓').await {
                            println!("Error reacting to message: {:?}", why);
                        }
                        message = msg
                            .channel_id
                            .messages(&ctx.http, GetMessages::new().limit(1))
                            .await
                            .unwrap()
                            .pop()
                            .unwrap();
                    } else if solution.contains(&message.content.to_lowercase()) {
                        if let Err(why) = message.react(&ctx.http, '✅').await {
                            println!("Error reacting to message: {:?}", why);
                        }
                        correct = true;
                        let embed = CreateEmbed::new()
                            .title("Correct!")
                            .description(format!("Solution: {}", solution));
                        let builder = CreateMessage::new().content("").tts(false).embed(embed);
                        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                    } else {
                        if let Err(why) = message.react(&ctx.http, '❌').await {
                            println!("Error reacting to message: {:?}", why);
                        }
                        message = msg
                            .channel_id
                            .messages(&ctx.http, GetMessages::new().limit(1))
                            .await
                            .unwrap()
                            .pop()
                            .unwrap();
                    }
                }
                println!("{}", timeout);
                // Delete ./puzzle.png
                std::fs::remove_file("./puzzle.png").unwrap();
            } else {
                println!("Unknown command");
                let embed = CreateEmbed::new()
                    .title("Unknown command")
                    .description("Use !help to see a list of commands");
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        let mudae_id =
            env::var("MUDAE_ID").expect("Failed to get MUDAE_ID from the environment variables");
        if msg.author.id == mudae_id.parse::<u64>().unwrap() {
            if let Some(embed) = msg.embeds.first() {
                let name = embed.author.as_ref().unwrap().name.clone();
                let data = include_str!("data.txt");
                let mut line_number = 0;
                let mut in_list = false;
                for line in data.lines() {
                    line_number += 1;
                    if line.to_lowercase() == name.to_lowercase() {
                        in_list = true;
                        let user_id = env::var("USER_ID")
                            .expect("Failed to get USER_ID from the environment variables");
                        let user = UserId::new(user_id.parse::<u64>().unwrap());
                        if let Err(why) = user
                            .create_dm_channel(&ctx.http)
                            .await
                            .unwrap()
                            .say(
                                &ctx.http,
                                format!(
                                    "{} is number {} in the list\n[Message]({})",
                                    name,
                                    line_number,
                                    msg.link()
                                ),
                            )
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
                if !in_list {
                    println!("{} is not in the list", name);
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Failed to get DISCORD_TOKEN from the environment variables");

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
