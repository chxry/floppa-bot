use discord::builders::EmbedFooterBuilder;
use discord::model::Message;

pub fn footer(b: EmbedFooterBuilder, msg: &Message) -> EmbedFooterBuilder {
  b.text(format!("Requested by {}", msg.author.name).as_str())
    .icon_url(
      &msg.author.avatar_url().unwrap_or(
        "https://discordapp.com/assets/dd4dbc0016779df1378e7812eabaa04d.png".to_string(),
      ),
    )
}
