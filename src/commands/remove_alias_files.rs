use anyhow::{Error};
use clap::Args;

use crate::{alias_handler::{remove_alias_file, rmeove_all_alias_files}, commands::Command};

#[derive(Args)]
pub struct RemoveArg {
  #[arg(long, short)]
  all: bool,
}

impl Command for RemoveArg {
  fn execute(&self) -> Result<(), Error> {
    if self.all {
      rmeove_all_alias_files()
    }
    else {
      remove_alias_file()
    }
  }
}
