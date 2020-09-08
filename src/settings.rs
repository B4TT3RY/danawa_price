use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Telegram {
    pub bot_token: String,
    pub chat_id: String,
    pub update_chat_description: bool,
}

#[derive(Debug, Deserialize)]
pub struct Danawa {
    pub url: String,
    pub product_list: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub telegram: Telegram,
    pub danawa: Danawa,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Settings"))?;
        s.try_into()
    }
}