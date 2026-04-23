use anyhow::Error;
use clap::Args;

use crate::{alias_handler::AliasManager, commands::Command};

#[derive(Args)]
pub struct RenameArg {
  old_alias: String,
  new_alias: String,
}

impl Command for RenameArg {
  fn execute(&self) -> Result<(), Error> {
    let alias_manager = AliasManager::new()?;
    let result = alias_manager.rename(&self.old_alias, &self.new_alias);

    if let Ok(()) = &result {
      println!("Renamed '{} => {}'", &self.old_alias, self.new_alias);
    }

    result
  }
}
