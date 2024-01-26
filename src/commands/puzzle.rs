// Desc: Play a random chess puzzle
use image::GenericImageView;
use pgnparse::parser::*;
use rand::seq::SliceRandom;
use reqwest::{get, Response};
use serde_json::Value;
use serenity::{
    builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage, GetMessages},
    model::channel::Message,
    prelude::*,
};
use shakmaty::*;

pub async fn puzzle(msg: &Message, ctx: &Context) {
    if msg.content == "!sol" || msg.content == "!solution" {
        let file_path = std::path::Path::new("puzzle.png");
        if file_path.exists() {
            println!("Sending solution");
            let solution = std::env::var("SOLUTION").unwrap();
            let puzzle_id = std::env::var("PUZZLE_ID").unwrap();
            let embed = CreateEmbed::new().title("Solution").description(format!(
                "Solution: {}\n\n[lichess](https://lichess.com/training/{})",
                solution, puzzle_id
            ));
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            let _ = msg.channel_id.send_message(&ctx.http, builder).await;
            std::fs::remove_file("puzzle.png").unwrap();
            std::env::set_var("PUZZLE_ID", "");
            std::env::set_var("SOLUTION", "");
            return;
        } else {
            println!("No puzzle in progress");
            let embed = CreateEmbed::new()
                .title("No puzzle in progress")
                .description("Use !puzzle to start a new puzzle");
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error sending message: {:?}", why);
            }
            return;
        }
    }
    let file_path = std::path::Path::new("puzzle.png");
    if file_path.exists() {
        println!("Puzzle already in progress");
        let embed = CreateEmbed::new()
            .title("Puzzle already in progress")
            .description("Solve the puzzle, send \"!sol\", or wait for the timeout (90s)");
        let builder = CreateMessage::new().content("").tts(false).embed(embed);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error sending message: {:?}", why);
        }
        return;
    }
    let puzzle_player_id = msg.author.id.to_string();
    let file_contents = include_str!("data/puzzles.csv");
    let lines = file_contents.lines().collect::<Vec<&str>>();
    let line = lines.choose(&mut rand::thread_rng()).unwrap();
    std::env::set_var("PUZZLE_ID", line);
    println!("Sending puzzle {}", line);
    let url = format!("https://lichess.org/api/puzzle/{}", line);
    let json: Value = get(&url).await.unwrap().json().await.unwrap();
    let fen = parse_pgn_to_rust_struct(json["game"]["pgn"].as_str().unwrap())
        .moves
        .last()
        .unwrap()
        .fen_after
        .clone();
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
    let solution = json["puzzle"]["solution"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<Vec<&str>>();
    let solution = solution.join(" ");
    let url = format!("https://fen2image.chessvision.ai/{}", fen);
    let response: Response = get(&url).await.unwrap();
    let image = response.bytes().await.unwrap();
    let image = image::load_from_memory(&image).unwrap();
    let (width, _) = image.dimensions();
    let image = image.crop_imm(0, 0, width, width);
    image.save("puzzle.png").unwrap();
    let attachment = CreateAttachment::path("puzzle.png").await;
    let footer = CreateEmbedFooter::new(format!("Puzzle ID: {}", line));
    let embed = CreateEmbed::new()
        .title("Puzzle")
        .footer(footer)
        .description(format!(
            "{} to move. Input a move in UCI format (example: b4b5, d7e8, etc.)\n\n[lichess](https://lichess.com/training/{})",
            whose_turn, line
        ))
        .attachment("puzzle.png");
    let builder = CreateMessage::new().content("").tts(false).embed(embed);
    let _ = msg
        .channel_id
        .send_files(&ctx.http, attachment, builder)
        .await
        .unwrap();
    let board = shakmaty::Board::from_ascii_board_fen(board_fen.as_bytes()).unwrap();
    println!("{:?}", board.occupied());
    let mut setup = Setup::empty();
    setup.board = board;
    setup.turn = board_turn;
    let mode = shakmaty::CastlingMode::Standard;
    let mut pos = Chess::from_setup(setup, mode).unwrap();
    let legals = pos.legal_moves();
    let mut uci_legals = Vec::new();
    let mut san_legals = Vec::new();
    let mut san_strings = Vec::new();
    for legal in legals {
        uci_legals.push(legal.to_string());
    }
    for uci_legal in &mut uci_legals {
        *uci_legal = uci_legal.replace("x", "");
        *uci_legal = uci_legal.replace("-", "");
        *uci_legal = uci_legal.replace("N", "");
        *uci_legal = uci_legal.replace("B", "");
        *uci_legal = uci_legal.replace("R", "");
        *uci_legal = uci_legal.replace("Q", "");
        *uci_legal = uci_legal.replace("K", "");
    }
    let new_legals = pos.legal_moves();
    for legal in new_legals {
        san_legals.push(shakmaty::san::San::from_move(&pos, &legal));
    }
    for san_legal in &mut san_legals {
        san_strings.push(san_legal.to_string());
    }
    println!("Legal moves (UCI): {:?}", uci_legals);
    println!("Legal moves (SAN): {:?}", san_strings);
    let mut correct = false;
    let mut timeout = false;
    let original_solution = solution.clone();
    let mut solution = solution.split(' ').collect::<Vec<&str>>();
    let mut next_move = solution[0];
    let time = std::time::Instant::now();
    println!("Solution: {}", solution.join(" "));
    std::env::set_var("SOLUTION", solution.join(" "));
    while correct == false && timeout == false {
        println!("{}", time.elapsed().as_secs_f32());
        if time.elapsed().as_secs_f32() > 90.0 {
            if let Err(why) = msg.react(&ctx.http, '⏰').await {
                println!("Error reacting to message: {:?}", why);
            }
            let embed = CreateEmbed::new()
                .title("Time's up!")
                .description(format!("Solution: {}", original_solution));
            let builder = CreateMessage::new().content("").tts(false).embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("Error sending message: {:?}", why);
            }
            timeout = true;
            continue;
        }
        let mut messages = msg
            .channel_id
            .messages(&ctx.http, GetMessages::new().limit(1))
            .await
            .unwrap();
        let message = messages.pop().unwrap();
        if message.author.id.to_string() != puzzle_player_id {
            continue;
        }
        if !uci_legals.contains(&message.content.to_lowercase()) {
            if let Err(why) = message.react(&ctx.http, '❓').await {
                println!("Error reacting to message: {:?}", why);
            }
            continue;
        } else if &message.content.to_lowercase() == next_move {
            if let Err(why) = message.react(&ctx.http, '✅').await {
                println!("Error reacting to message: {:?}", why);
            }
            next_move = solution[0];
            solution = solution[1..].to_vec();
            if solution.len() == 0 {
                correct = true;
                let attachment = CreateAttachment::path("./puzzle.png").await;
                let embed = CreateEmbed::new()
                    .title("Correct!")
                    .description(format!("Solution: {}", original_solution))
                    .attachment("puzzle.png");
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, attachment, builder)
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                println!("Solution before opponent move: {}", solution.join(" "));
                println!("Your move: {}", next_move);
                let uci_move: uci::Uci = next_move.parse().unwrap();
                let m = uci_move.to_move(&pos).unwrap();
                pos.play_unchecked(&m);
                next_move = solution[0];
                solution = solution[1..].to_vec();
                println!("Solution after opponent move: {}", solution.join(" "));
                println!("Opponent move: {}", next_move);
                let uci_move: uci::Uci = next_move.parse().expect("Invalid move");
                println!("UCI move: {}", uci_move);
                let m = uci_move.to_move(&pos).expect("Invalid move");
                pos.play_unchecked(&m);
                next_move = solution[0];
                let board_fen = Board::board_fen(&pos.board(), pos.promoted());
                println!("{}", board_fen.to_string());
                let board = shakmaty::Board::from_ascii_board_fen(board_fen.to_string().as_bytes())
                    .unwrap();
                println!("{:?}", board.occupied());
                setup = Setup::empty();
                setup.board = board;
                setup.turn = board_turn;
                let pos_new = Chess::from_setup(setup, mode).expect("Invalid FEN");
                let legals = pos_new.legal_moves();
                uci_legals = Vec::new();
                for legal in legals {
                    uci_legals.push(legal.to_string());
                }
                for uci_legal in &mut uci_legals {
                    *uci_legal = uci_legal.replace("x", "");
                    *uci_legal = uci_legal.replace("-", "");
                    *uci_legal = uci_legal.replace("N", "");
                    *uci_legal = uci_legal.replace("B", "");
                    *uci_legal = uci_legal.replace("R", "");
                    *uci_legal = uci_legal.replace("Q", "");
                    *uci_legal = uci_legal.replace("K", "");
                }
                println!("Legal moves: {:?}", uci_legals);
                let url = format!("https://fen2image.chessvision.ai/{}", board_fen);
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
                let builder = CreateMessage::new().content("").tts(false).embed(embed);
                let _ = msg
                    .channel_id
                    .send_files(&ctx.http, attachment, builder)
                    .await
                    .unwrap();
                pos = pos_new;
            }
        } else {
            if let Err(why) = message.react(&ctx.http, '❌').await {
                println!("Error reacting to message: {:?}", why);
            }
            continue;
        }
    }
    println!("{}", timeout);
    std::fs::remove_file("./puzzle.png").unwrap();
}
