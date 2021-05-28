mod caption;
mod help;

use std::collections::HashMap;
use discord::Discord;
use discord::model::Message;
use log::error;
use crate::embed::footer;

fn remove_first(value: &str) -> &str {
  let mut chars = value.chars();
  chars.next();
  chars.as_str()
}

type CommandFn =
  fn(discord: &Discord, msg: &Message, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;

pub struct Commands<'a> {
  commands: HashMap<&'a str, Box<CommandFn>>,
}

impl<'a> Commands<'a> {
  pub fn new() -> Self {
    Self {
      commands: HashMap::new(),
    }
  }

  pub fn register_commands(&mut self) {
    self.register("help", help::help);
    self.register("caption", caption::caption);
  }

  fn register(&mut self, fn_name: &'a str, function: CommandFn) {
    self.commands.insert(fn_name, Box::new(function));
  }

  pub fn run(&mut self, discord: &Discord, msg: &Message) {
    let args: Vec<&str> = remove_first(&msg.content).split_whitespace().collect();
    if args.len() == 0 {
      return;
    }
    let cmd_name = &*args[0].to_lowercase();
    let cmd = match self.commands.get(cmd_name) {
      Some(cmd) => cmd,
      _ => return,
    };

    let res = cmd(&discord, &msg, args);
    if res.is_err() {
      let err = res.unwrap_err();
      error!("Error occoured in command {:?}.", err);
      discord
        .send_embed(msg.channel_id, "", |b| {
          b.title("An error occoured.")
            .description(format!("```rs\n{:?}```", err).as_str())
            .footer(|b| footer(b, msg))
        })
        .unwrap();
    }
  }
}
