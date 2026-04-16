use clap::{ Parser, Subcommand };
mod command;
mod register;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {

}

fn main() {
    println!("Hello, world!");
}
