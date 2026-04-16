use clap::{ Parser, Subcommand };
use crate::command::Command;

mod command;
mod register;


#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Commands
}

#[derive(Subcommand)]
enum Commands {

}

fn main() {
    println!("Hello, world!");
}
