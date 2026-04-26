use clap::{ Parser };
use std::process;

use crate::commands::{Commands};

mod commands;
mod alias_handler;
mod tmux_handler;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

fn main() {
  let cli = Cli::parse();

  let command_result = cli.command.run();

  if let Err(e) = command_result {
    eprintln!("{:?}", e);

    process::exit(1);
  }
}
