use std::process;
use log::{error, LevelFilter};
use simplelog::{WriteLogger, Config};
use std::fs::File;
use discord_name_color::bot_config;

fn main() {
    if let Err(why) = WriteLogger::init(LevelFilter::Info, Config::default(), File::create(&bot_config::CONFIG.logfile).unwrap()) {
        eprintln!("Failed to intitialize logger: {:?}", why);
        process::exit(1);
    }
    if let Err(why) = discord_name_color::run() {
        error!("An error occurred while starting the client: {:?}", why);
        process::exit(1);
    }
}