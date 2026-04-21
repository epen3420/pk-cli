use anyhow::Error;
use clap::Args;
use std::path::PathBuf;

use crate::{alias_handler, command::Command};

#[derive(Args)]
pub struct RegisterArg {
  path: PathBuf,
  alias: String,
}

impl Command for RegisterArg {
  fn execute(&self) -> Result<(), Error> {
    let result = alias_handler::create(&self.alias, &self.path);

    if let Ok(()) = &result {
      println!("Registered '{} => {}'", &self.alias, self.path.display());
    }

    result
  }
}
