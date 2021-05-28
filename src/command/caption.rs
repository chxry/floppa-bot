use std::env::current_dir;
use std::error::Error;
use std::process::Command;
use std::fs;
use std::io::Write;
use discord::Discord;
use discord::model::Message;
use imagesize::size;
use log::info;
use crate::embed::footer;

pub fn caption(
  discord: &Discord,
  msg: &Message,
  mut args: Vec<&str>,
) -> Result<(), Box<dyn Error>> {
  let caption_id = msg.channel_id.to_string() + &msg.id.to_string();
  info!(
    "Generating caption {} for {}#{}.",
    &caption_id, msg.author.name, msg.author.discriminator
  );
  args.drain(0..1);

  if args.len() == 0 {
    discord.send_embed(msg.channel_id, "", |b| {
      b.title("No caption provided.").footer(|b| footer(b, msg))
    })?;
    return Ok(());
  }

  if msg.attachments.len() == 0 {
    discord.send_embed(msg.channel_id, "", |b| {
      b.title("No file attached.").footer(|b| footer(b, msg))
    })?;
    return Ok(());
  }

  let img_bytes = reqwest::blocking::get(&msg.attachments[0].url)?.bytes()?;

  let processing_msg = discord.send_embed(msg.channel_id, "", |b| {
    b.title("Processing.")
      .description("This may take a while...")
      .footer(|b| footer(b, msg))
  })?;

  let mut temp_path = current_dir()?;
  temp_path.push("tmp");
  temp_path.push(&caption_id);
  fs::create_dir_all(&temp_path)?;

  let mut input_path = temp_path.clone();
  input_path.push("input.gif");
  fs::File::create(&input_path)?.write_all(&img_bytes)?;

  let mut caption_path = temp_path.clone();
  caption_path.push("caption.png");

  let mut output_path = temp_path.clone();
  output_path.push("output.gif");

  //Generate caption
  let input_dimensions = size(input_path.clone().into_os_string())?;
  let point_size = input_dimensions.width / 9;
  Command::new("convert")
    .arg("-background")
    .arg("white")
    .arg("-interline-spacing")
    .arg("0")
    .arg("-kerning")
    .arg("0")
    .arg("-fill")
    .arg("black")
    .arg("-font")
    .arg(format!("{}/resources/futura.otf", current_dir()?.display()))
    .arg("-pointsize")
    .arg(&point_size.to_string().as_str())
    .arg("-size")
    .arg(format!("{}x", input_dimensions.width - point_size))
    .arg("-gravity")
    .arg("Center")
    .arg(format!("caption:{}", args.join(" ")))
    .arg(format!("PNG24:{}", caption_path.display()))
    .status()?;

  //Apply caption padding
  let caption_dimensions = size(caption_path.clone().into_os_string())?;
  Command::new("convert")
    .arg(format!("{}", caption_path.display()))
    .arg("-gravity")
    .arg("Center")
    .arg("-extent")
    .arg(format!(
      "{}x{}",
      caption_dimensions.width + point_size,
      caption_dimensions.height + point_size
    ))
    .arg(format!("PNG24:{}", caption_path.display()))
    .status()?;

  //Merge images
  Command::new("convert")
    .arg(caption_path.clone().into_os_string())
    .arg("(")
    .arg(input_path.clone().into_os_string())
    .arg("-coalesce")
    .arg(")")
    .arg("-set")
    .arg("page")
    .arg("%[fx:u.w]x%[fx:u.h+v.h]+%[fx:t?(u.w-v.w)/2:0]+%[fx:t?u.h:0]")
    .arg("-coalesce")
    .arg("null:")
    .arg("-insert")
    .arg("1")
    .arg("-layers")
    .arg("composite")
    .arg(output_path.clone().into_os_string())
    .status()?;

  discord.send_file(
    msg.channel_id,
    "",
    fs::File::open(output_path.into_os_string())?,
    "floppa-generated.gif",
  )?;
  discord.delete_message(processing_msg.channel_id, processing_msg.id)?;

  fs::remove_dir_all(&temp_path)?;

  info!("Caption {} completed.", caption_id);
  Ok(())
}
