use config::ConfigError;
use std::process;

pub struct BotConfig {
    pub prefix: String,
    pub token: String
}

pub fn get_config() -> Result<BotConfig, ConfigError> {
    let mut settings = config::Config::default();
    settings.set_default("prefix","!")?;

    if let Err(why) = settings.merge(config::File::with_name("config.yaml")) {
        eprintln!("Error loading config.yaml: {}", why);
        process::exit(1);
    }

    let prefix = settings.get_str("prefix").expect("Error getting prefix");
    let token = settings.get_str("token").expect("Error getting token");
    let cfg = BotConfig { prefix, token };

    Ok(cfg)
}