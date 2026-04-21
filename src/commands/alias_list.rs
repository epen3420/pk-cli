use clap::Args;

use crate::{alias_handler, commands::Command};

#[derive(Args)]
pub struct AliasListArg {

}

impl Command for AliasListArg {
  fn execute(&self) -> Result<(), anyhow::Error> {
    alias_handler::show_list()
  }
}
