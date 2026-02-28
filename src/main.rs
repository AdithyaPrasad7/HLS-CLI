mod commands;
mod logger;

use clap::Parser;

fn main() {
  logger::init_logging();

  let args = commands::cli::Args::parse();
}
