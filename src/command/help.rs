use std::error::Error;
use discord::Discord;
use discord::model::Message;
use crate::embed::footer;

pub fn help(discord: &Discord, msg: &Message, _: Vec<&str>) -> Result<(), Box<dyn Error>> {
  discord.send_embed(msg.channel_id, "", |b| {
    b.title("Help:")
      .description("List of commands:\n*URL* can be ommmited and an image attached.")
      .fields(|b| {
        b.field(
          "**- !caption**",
          "Adds a caption to a GIF or image\n **Usage:** !caption *URL* text",
          false,
        )
        .field(
          "**- !help**",
          "Displays this menu\n **Usage:** !help",
          false,
        )
        .field(
          "**- !info**",
          "Displays bot information\n **Usage:** !info",
          false,
        )
      })
      .footer(|b| footer(b, msg))
  })?;
  Ok(())
}
