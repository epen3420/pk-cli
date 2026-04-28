use anyhow::Error;
use clap::Args;

use crate::{alias_handler, commands::Command, input_util, tmux_handler};

#[derive(Args)]
pub struct RunArg {
  alias: Option<String>,
}

impl Command for RunArg {
  fn execute(&self) -> Result<(), Error> {
    let alaias_man = alias_handler::AliasManager::new()?;

    if let Some(alias) = &self.alias {
      let path = alaias_man.get_path_by_alias(&alias)?;
      return tmux_handler::create_session(alias, &path);
    }

    alaias_man.show_list_with_num()?;
    let num = input_util::get_input();

    let (alias, path) = alaias_man.get_alias_and_path_by_num(num)?;
    tmux_handler::create_session(&alias, &path)
  }
}
