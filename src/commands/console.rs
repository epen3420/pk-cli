use anyhow::Error;
use clap::Args;

use crate::{tmux_handler, commands::Command};

#[derive(Args)]
pub struct ConsoleArg {
  alias: String,
}

impl Command for ConsoleArg {
  fn execute(&self) -> Result<(), Error> {
    tmux_handler::attach_session(&self.alias)
  }
}
