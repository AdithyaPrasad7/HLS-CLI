use clap::{Parser, Subcommand};
use crate::model::resolutionDetails::ResolutionDetails;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  Auth {
    #[arg(long)]
    token: String,
  },
  Transcode {
    #[arg(long)]
    inputPath: String,

    #[arg(long)]
    outputPath: Option<String>,

    #[arg(long, value_delimiter = ',')]
    resolutions: Option<Vec<ResolutionDetails>>,
  },
}