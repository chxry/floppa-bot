use std::error::Error;
use std::process::{id, Command};
use discord::Discord;
use discord::model::Message;
use crate::embed::footer;

pub fn info(discord: &Discord, msg: &Message, _: Vec<&str>) -> Result<(), Box<dyn Error>> {
  let mut uptime = String::from_utf8(
    Command::new("ps")
      .arg("-p")
      .arg(id().to_string())
      .arg("-o")
      .arg("etimes=")
      .output()?
      .stdout,
  )?;
  uptime.retain(|c| !c.is_whitespace());
  let uptime = uptime.parse::<u32>()?;

  let seconds = uptime % 60;
  let minutes = (uptime / 60) % 60;
  let hours = (uptime / (60 * 60)) % 24;

  discord.send_embed(msg.channel_id, "", |b| {
    b.title("Info:")
      .description("About the bot.")
      .fields(|b| {
        b.field(
          "**Uptime**",
          format!("{:02}:{:02}:{:02}", hours, minutes, seconds).as_str(),
          true,
        )
        .field(
          "**Version**",
          format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
          true,
        )
      })
      .footer(|b| footer(b, msg))
  })?;
  Ok(())
}
