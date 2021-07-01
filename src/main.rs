use std::process;
use log::{error, LevelFilter};
use simplelog::{WriteLogger, Config};
use std::fs::File;
use discord_name_color::bot_config;

#[tokio::main]
async fn main() {
    if let Err(why) = WriteLogger::init(LevelFilter::Warn, Config::default(), File::create(&bot_config::CONFIG.logfile).unwrap()) {
        eprintln!("Failed to intitialize logger: {:?}", why);
        process::exit(1);
    }
    if let Err(why) = discord_name_color::run().await {
        error!("An error occurred while starting the client: {:?}", why);
        process::exit(1);
    }
}