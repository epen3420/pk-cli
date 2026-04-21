use clap::{ Parser };
use std::process;

use crate::commands::{Commands};

mod commands;
mod alias_handler;


#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Commands,

  #[cfg(debug_assertions)]
  #[arg(long)]
  delete: bool,
}

fn main() {
  let cli = Cli::parse();

  let command_result = cli.command.run();

  #[cfg(debug_assertions)]
  if cli.delete {
    let _ = alias_handler::remove_alias_files();
    println!("removed all alias files");
  }

  if let Err(e) = command_result {
    eprintln!("{:?}", e);

    process::exit(1);
  }
}
