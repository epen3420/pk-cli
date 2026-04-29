use anyhow::Error;
use clap::Args;

use crate::{alias_handler::AliasManager, commands::Command};

#[derive(Args)]
pub struct DeleteArg {
  alias: String,
}

impl Command for DeleteArg {
  fn execute(&self) -> Result<(), Error> {
    let alias_manager = AliasManager::new()?;
    let result = alias_manager.delete(&self.alias);

    if let Ok(()) = &result {
      println!("Deleted '{}'", &self.alias);
    }

    result
  }
}
