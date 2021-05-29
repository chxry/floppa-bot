mod command;
mod config;
mod embed;

use std::error::Error;
use discord::Discord;
use discord::model::Event;
use command::Commands;
use config::BotConfig;
use log::{info, error};
use simplelog::*;

fn main() -> Result<(), Box<dyn Error>> {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )?;

  let config = BotConfig::load();
  info!(
    "Loaded {} v{}.",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
  let discord = Discord::from_bot_token(config.token)?;

  let mut commands = Commands::new();
  commands.register_commands();
  info!("Loaded commands: {}", commands);

  let (mut connection, _) = discord.connect()?;
  info!("Connected.");

  loop {
    match connection.recv_event() {
      Ok(Event::MessageCreate(message)) => {
        if message.content.starts_with(config.prefix) {
          commands.run(&discord, &message);
        }
      }
      Ok(_) => {}
      Err(err) => error!("Received error: {}.", err),
    }
  }
}
