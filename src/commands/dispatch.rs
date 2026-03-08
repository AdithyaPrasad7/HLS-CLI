use tracing::{info, error};
use std::arch::aarch64::vld1_dup_u32;
use std::path::{Path, PathBuf};

use crate::commands::cli::{Args, Commands};
use crate::commands::run::transcodeWithQuality;
use crate::model::resolutionDetails::ResolutionDetails;
use crate::config::Error;
use crate::token::CONFIG;
use crate::client::service::{validateToken, getAWSSession, updateVideoDetails};
use crate::aws::{s3Client, uploadFiles};

pub async fn execute(args: Args) -> Result<(), Error> {
  match args.command {
    Commands::Auth { token } => {
      let response = validateToken(&token).await?;
      CONFIG.setToken(token.as_str(), response);
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

      match transcodeWithQuality(inputPath, finalOutputPath.clone(), resolution) {
        Ok(_) => {
          info!("Transcoding completed successfully.");
          println!("Transcoding completed successfully.");
        },
        Err(e) => {
          error!("Transcoding failed: {}", e);
          return Err(e);
        },
      }
      let isTokenValid= CONFIG.isTokenValid();
      if isTokenValid {
        info!("Starting to upload the transcoded files");
        println!("Starting to upload the transcoded files");
        let credentials= getAWSSession().await?;
        let s3Client = s3Client::s3Client(credentials.credentials).await;
        match uploadFiles::uploadFiles(&s3Client, &credentials.bucket, &credentials.path, &finalOutputPath).await {
          Ok(_) => {
            updateVideoDetails(credentials.path, true).await?;
            println!("Succesfully transcoded and uploaded the files");
            info!("Succesfully transcoded and uploaded the files");
          }

          Err(e) => {
            updateVideoDetails(credentials.path, false).await?;
            return Err(e);
          }
        }
      } else {
        info!("Token is invalid");
        println!("Token is invalid");
      }
      Ok(())
    }
  }
}