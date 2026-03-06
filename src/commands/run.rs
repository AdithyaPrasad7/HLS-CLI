use std::fs::File;
use std::io::Write;
use tracing::{info, error};
use std::thread;
use std::sync::Arc;

use crate::ffmpeg::transcode::FFmpegTranscoder;
use crate::model::resolutionDetails::ResolutionDetails;
use crate::config::DynError;
use crate::config::Error;


pub fn transcodeWithQuality(
  inputPath: String,
  outputPath: String,
  resolutions: Vec<ResolutionDetails>
) -> Result<(), Error> {
  let input = Arc::new(inputPath);
  let output = Arc::new(outputPath);
  let mut handles = Vec::new();

  for &resolution in &resolutions {
    let inputClone = Arc::clone(&input);
    let outputClone = Arc::clone(&output);
    let handle = thread::spawn(move || -> Result<(), DynError> {
      FFmpegTranscoder::hlsTranscode(
        &inputClone,
        &format!("{}/{}", outputClone, resolution.name()),
        resolution.height(),
        resolution.width(),
      )?;

      Ok(())
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  generateMasterPlaylist(&output, &resolutions);

  Ok(())
}

fn generateMasterPlaylist(outputPath: &str, resolutions: &Vec<ResolutionDetails>) -> std::io::Result<()> {
  let mut file = File::create(format!("{}/master.m3u8", outputPath))?;
  writeln!(file, "#EXTM3U")?;
  
  for resolution in resolutions {
    writeln!(file, "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{}", resolution.bandwidth(), resolution.width(), resolution.height())?;

    writeln!(file, "{}/playlist.m3u8", resolution.name())?;
  }

  info!("Master playlist created");

  Ok(())
}