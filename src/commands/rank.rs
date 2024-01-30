// Desc: Get Rocket League rank (TODO)
use dotenvy::dotenv;
use image::GenericImageView;
use pgnparse::parser::*;
use rand::seq::SliceRandom;
use ratcurl::easy::{self, Easy, List};
use reqwest::{get, header, Response};
use serde_json::Value;
use serenity::framework::standard::{Args, CommandResult};
use serenity::{
    async_trait,
    builder::{CreateAttachment, CreateEmbed, CreateMessage, GetMessages},
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use std::env;
use std::io::{stdout, Read, Write};

pub async fn rlrank(msg: &Message, ctx: &Context) {
    println!("Getting Rocket League rank");
    // Command will come in the form of !rlrank <username> <platform>
    // Command might be !rlrank or !r
    let mut content = msg.content[3..].trim();
    if msg.content.starts_with("!rlrank") {
        content = msg.content[8..].trim();
    }
    let mut args = content.split_whitespace();
    let username = args.next().unwrap();
    let platform = args.next().unwrap();
    // Print both
    println!("Username: {}", username);
    println!("Platform: {}", platform);
    // Get the user's Rocket League ID
    let mut url = "https://api.tracker.gg/api/v2/rocket-league/standard/profile/".to_string();
    url.push_str(platform);
    url.push_str("/");
    url.push_str(username);
}
