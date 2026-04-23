use clap::Args;

use crate::{alias_handler::AliasManager, commands::Command};

#[derive(Args)]
pub struct AliasListArg {

}

impl Command for AliasListArg {
  fn execute(&self) -> Result<(), anyhow::Error> {
    let alias_manager = AliasManager::new()?;
    alias_manager.show_list()
  }
}
