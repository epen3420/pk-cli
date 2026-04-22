#![allow(dead_code)]

use clap::Subcommand;

mod register;
mod rename;
mod delete;
mod alias_list;
mod remove_alias_files;


pub trait Command {
  fn execute(&self) -> Result<(), anyhow::Error>;
}

#[derive(Subcommand)]
pub enum Commands {
  Register(register::RegisterArg),
  Rename(rename::RenameArg),
  Delete(delete::DeleteArg),
  List(alias_list::AliasListArg),

  #[cfg(debug_assertions)]
  Remove(remove_alias_files::RemoveArg),
}

impl Commands {
  pub fn run(&self) -> Result<(), anyhow::Error> {
    match self {
      Self::Register(arg) => arg.execute(),
      Self::Rename(arg) => arg.execute(),
      Self::Delete(arg) => arg.execute(),
      Self::List(arg) => arg.execute(),

      #[cfg(debug_assertions)]
      Self::Remove(arg) => arg.execute(),
    }
  }
}
