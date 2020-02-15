use config::ConfigError;
use std::process;

pub struct BotConfig {
    pub prefix: String,
    pub token: String,
    pub channel_ids: Vec<u64>,
    pub logfile: String
}

pub fn get_config() -> Result<BotConfig, ConfigError> {
    let mut settings = config::Config::default();
    settings.set_default("prefix","!")?;
    settings.set_default("channel_ids", Vec::<i64>::new())?;
    settings.set_default("logfile", "colorbot.log")?;

    if let Err(why) = settings.merge(config::File::with_name("config.yaml")) {
        eprintln!("Error loading config.yaml: {}", why);
        process::exit(1);
    }

    let prefix = settings.get_str("prefix").expect("Error getting prefix");
    let token = settings.get_str("token").expect("Error getting token");
    let channel_ids = settings.get::<Vec<u64>>("channel_ids").expect("Error getting channel ids");
    let logfile = settings.get_str("logfile").expect("Error getting logfile");

    let cfg = BotConfig { prefix, token, channel_ids, logfile };

    Ok(cfg)
}

lazy_static! {
    pub static ref CONFIG: BotConfig = {
        get_config().expect("Failed to load config!")
    };
}