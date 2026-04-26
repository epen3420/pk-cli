use anyhow::Error;
use clap::Args;

use crate::{alias_handler, commands::Command, tmux_handler};

#[derive(Args)]
pub struct RunArg {
  alias: String,
}

impl Command for RunArg {
  fn execute(&self) -> Result<(), Error> {
    let alaias_man = alias_handler::AliasManager::new()?;

    tmux_handler::create_session(&self.alias, &alaias_man.get_path_by_alias(&self.alias)?)
  }
}
