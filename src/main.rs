mod commands;
mod logger;
mod client;
mod model;
mod config;
mod token;

use clap::Parser;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
  logger::init_logging();

  let args = commands::cli::Args::parse();

  pub static TOKEN_MGR: Lazy<Arc<token::TokenManager>> = Lazy::new(|| {
    Arc::new(token::TokenManager::new("HLS-CLI", "default"))
});

  Ok(())
}
