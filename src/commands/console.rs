use anyhow::Error;
use clap::Args;

use crate::{tmux_handler, commands::Command};

#[derive(Args)]
pub struct ConsoleArg {
  alias: Option<String>,
}

impl Command for ConsoleArg {
  fn execute(&self) -> Result<(), Error> {
    if let Some(alias) = &self.alias {
      return tmux_handler::attach_session(alias);
    }


    Ok(())
  }
}
