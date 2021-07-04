use config::ConfigError;
use std::process;

pub struct BotConfig {
    pub token: String,
    pub logfile: String,
    pub application_id: u64,
}

pub fn get_config() -> Result<BotConfig, ConfigError> {
    let mut settings = config::Config::default();
    settings.set_default("logfile", "colorbot.log")?;

    if let Err(why) = settings.merge(config::File::with_name("config.yaml")) {
        eprintln!("Error loading config.yaml: {}", why);
        process::exit(1);
    }

    let token = settings.get_str("token").expect("Error getting token");
    let logfile = settings.get_str("logfile").expect("Error getting logfile");
    let application_id = settings.get_str("application_id").expect("Error getting application id").parse::<u64>().unwrap();

    let cfg = BotConfig { token, logfile, application_id };

    Ok(cfg)
}

lazy_static! {
    pub static ref CONFIG: BotConfig = {
        get_config().expect("Failed to load config!")
    };
}