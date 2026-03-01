mod commands;
mod logger;
mod client;
mod model;
mod config;
mod token;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
  logger::init_logging();

  let args = commands::cli::Args::parse();

  Ok(())
}
