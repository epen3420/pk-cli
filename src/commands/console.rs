use anyhow::Error;
use clap::Args;

use crate::{commands::Command, tmux_handler};

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

    tmux_handler::attach_session_interactive()
  }
}
