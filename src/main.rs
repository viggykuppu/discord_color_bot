use std::env;
use std::process;


fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");

    // start listening for events by starting a single shard
    if let Err(why) = discord_name_color::run(&token) {
        eprintln!("An error occurred while running the client: {:?}", why);
        process::exit(1)
    }
}