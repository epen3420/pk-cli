use clap::{ Parser, Subcommand };
use crate::command::Command;
use std::process;

mod command;
mod alias_handler;
mod register;


#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Commands,

  #[cfg(debug_assertions)]
  #[arg(long)]
  delete: bool,
}

#[derive(Subcommand)]
enum Commands {
  Register(register::RegisterArg)
}

fn main() {
  let cli = Cli::parse();

  let command_result = match cli.command {
    Commands::Register(arg) => arg.execute(),
  };

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
