use discord_name_color::bot_config;
use std::process;


fn main() {
    //let token = env::var("DISCORD_TOKEN").expect("token");
    match bot_config::get_config() {
        Ok(config) => {            
            // start listening for events by starting a single shard
            if let Err(why) = discord_name_color::run(config) {
                eprintln!("An error occurred while starting the client: {:?}", why);
                process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("Error {:?}",e);
        }
    }
}