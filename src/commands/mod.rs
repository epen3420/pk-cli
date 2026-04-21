#![allow(dead_code)]

use clap::Subcommand;

mod register;


pub trait Command {
  fn execute(&self) -> Result<(), anyhow::Error>;
}

#[derive(Subcommand)]
pub enum Commands {
  Register(register::RegisterArg)
}

impl Commands {
  pub fn run(&self) -> Result<(), anyhow::Error> {
    match self {
      Self::Register(arg) => arg.execute(),
    }
  }
}
