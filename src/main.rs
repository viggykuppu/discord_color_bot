use std::env;
use std::process;
use config::{ Config, ConfigError };

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    match get_config() {
        Ok(config) => {
            println!("config: {:?}", config);
            if let Ok(prefix) = config.get_str("prefix") {
                println!("Prefix {}", prefix);
            }
        },
        Err(e) => {
            eprintln!("Error {:?}",e);
        }
    }

    // start listening for events by starting a single shard
    // if let Err(why) = discord_name_color::run(&token) {
    //     eprintln!("An error occurred while running the client: {:?}", why);
    //     process::exit(1)
    // }
}

fn get_config() -> Result<Config, ConfigError> {
    let mut settings = config::Config::default();
    settings.set_default("prefix","!")?;
    settings.merge(config::File::with_name("config.yaml"))?;

    Ok(settings)
}