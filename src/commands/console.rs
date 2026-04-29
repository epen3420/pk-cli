use anyhow::Error;
use clap::Args;

use crate::{commands::Command, input_util, tmux_handler};

#[derive(Args)]
pub struct ConsoleArg {
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
    let mut count = 0;
    for alias in &running_alias {
      println!("{}: {}", count, alias);
      count += 1;
    }

    let input_num = input_util::get_input();
    tmux_handler::attach_session(&running_alias[input_num])
  }
}
