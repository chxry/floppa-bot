use std::fs;
use std::str;
use std::process;
use serde::Deserialize;
use log::{info, error};

#[derive(Deserialize)]
pub struct BotConfig {
  pub token: &'static str,
  pub prefix: &'static str,
}

#[allow(unused_must_use)]
impl BotConfig {
  pub fn load() -> Self {
    let data = match fs::read_to_string("floppa-bot.toml") {
      Ok(data) => data,
      Err(_) => {
        let data = include_str!("default-config.toml");
        info!("Couldn't read config, generating one.");
        fs::write("floppa-bot.toml", data);
        data.to_string()
      }
    };
    let config: Self = match toml::from_str(Box::leak(data.into_boxed_str())) {
      Ok(config) => config,
      Err(_) => {
        error!("Could not parse config.");
        process::exit(1);
      }
    };
    config
  }
}
