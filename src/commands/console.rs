use anyhow::Error;
use clap::Args;

use crate::{commands::Command, input_util, tmux_handler};

#[derive(Args)]
pub struct ConsoleArg {
  /// The session alias to attach to. If omitted, prompts for a running session.
  alias: Option<String>,
}

impl Command for ConsoleArg {
  fn execute(&self) -> Result<(), Error> {
    if let Some(alias) = &self.alias {
      return tmux_handler::attach_session(alias);
    }

    let running_alias = tmux_handler::get_running_alias()?;

    if running_alias.len() == 1 {
      return tmux_handler::attach_session(&running_alias[0]);
    }

    println!("===== Current running sessions =====");
    let mut count = 1;
    for alias in &running_alias {
      println!("{}: {}", count, alias);
      count += 1;
    }
    println!();
    let num = input_util::get_input_num()?;
    let index = num - 1;

    if index <= 0 {
      return Ok(());
    }

    tmux_handler::attach_session(&running_alias[index])
  }
}
