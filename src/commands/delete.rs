use anyhow::Error;
use clap::Args;

use crate::{alias_handler, commands::Command};

#[derive(Args)]
pub struct DeleteArg {
  alias: String,
}

impl Command for DeleteArg {
  fn execute(&self) -> Result<(), Error> {
    let result = alias_handler::delete(&self.alias);

    if let Ok(()) = &result {
      println!("Deleted '{}'", &self.alias);
    }

    result
  }
}
