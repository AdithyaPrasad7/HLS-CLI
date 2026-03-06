mod commands;
mod logger;
mod client;
mod model;
mod config;
mod token;
mod ffmpeg;

use clap::Parser;
use std::sync::Arc;
use crate::config::Error;
use token::ConfigManager;

#[tokio::main]
async fn main() -> Result<(), Error> { 
  logger::init_logging();


  
  let args = commands::cli::Args::parse();
  commands::dispatch::execute(args);

  Ok(())
}
