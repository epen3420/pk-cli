use anyhow::{Error};
use clap::Args;

use crate::{alias_handler::AliasManager, commands::Command};

#[derive(Args)]
pub struct RemoveArg {
  #[arg(long, short)]
  all: bool,
}

impl Command for RemoveArg {
  fn execute(&self) -> Result<(), Error> {
    let alias_manager = AliasManager::new()?;
    if self.all {
      alias_manager.rmeove_all_alias_files()
    }
    else {
      alias_manager.remove_alias_file()
    }
  }
}
