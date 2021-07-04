#[macro_use]
extern crate lazy_static;

mod color_parser;
mod color_name_map;
mod discord_commands;
mod color_bot_handler;

pub mod bot_config;

use serenity::{
    client::{
        Client,
    },
    framework::standard::{
        StandardFramework
    }
};

pub async fn run() -> Result<(), serenity::Error> {
    let token = bot_config::CONFIG.token.clone();
    let application_id = bot_config::CONFIG.application_id.clone();

    let mut client = Client::builder(token)
        .event_handler(color_bot_handler::ColorBotHandler)
        .application_id(application_id)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");
        
    client.start().await
}