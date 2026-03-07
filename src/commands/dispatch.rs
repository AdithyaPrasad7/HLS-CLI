use tracing::{info, error};
use std::arch::aarch64::vld1_dup_u32;
use std::path::{Path, PathBuf};

use crate::commands::cli::{Args, Commands};
use crate::commands::run::transcodeWithQuality;
use crate::model::resolutionDetails::ResolutionDetails;
use crate::config::Error;
use crate::token::CONFIG;
use crate::client::service::validateToken;

pub async fn execute(args: Args) -> Result<(), Error> {
  match args.command {
    Commands::Auth { token } => {
      let response = validateToken(&token).await?;
      CONFIG.set_token(token.as_str(), response);
      Ok(())
    },

    Commands::Transcode { inputPath, outputPath, resolutions } => {
      let resolution = match resolutions {
        Some(r) => r,
        None => {
          ResolutionDetails::getAll()
        }
      };

      let finalOutputPath = match outputPath {
        Some(path) => path,
        None => {
          let input = Path::new(&inputPath);
          let stem = input.file_stem().unwrap().to_str().unwrap();
          let parent = input.parent().unwrap_or(Path::new(""));
          
          let mut new_path = PathBuf::from(parent);
          new_path.push(format!("{}_hls", stem));
          
          new_path.to_str().unwrap().to_string()
        }
      };

      match transcodeWithQuality(inputPath, finalOutputPath, resolution) {
        Ok(_) => {
          info!("Transcoding completed successfully.");
          Ok(())
        },
        Err(e) => {
          error!("Transcoding failed: {}", e);
          Err(e)
        },
      }
    }
  }
}