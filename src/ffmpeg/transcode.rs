use ez_ffmpeg::{FfmpegContext, Output};
use std::path::Path;
use tracing::{info, error};

use crate::config::DynError;

pub struct FFmpegTranscoder;

impl FFmpegTranscoder {
  pub fn hlsTranscode(inputPath: &str, outputPath: &str, height: u32, width: u32) -> Result<(), DynError> {
    std::fs::create_dir_all(outputPath);

    let outputDir = Path::new(outputPath).join("playlist.m3u8");

    FfmpegContext::builder()
      .input(inputPath)
      .output(
        Output::from(outputDir.to_str().unwrap())
          .set_format("hls")
          .set_video_codec_opt("vf", &format!("scale={}:{}:flags=lanczos", width, height))
          .set_format_opt("hls_time", "5")
          .set_format_opt("hls_playlist_type", "vod")
          .set_format_opt(
              "hls_segment_filename",
              &format!("{}/segment_%03d.ts", outputPath),
        )
          .set_video_codec("libx264")
          .set_audio_codec("aac")
          .set_video_codec_opt("crf", "23")
      )
      .build()?
      .start()?
      .wait()?;

    info!("Transcoded {} to {}", inputPath, height);
    Ok(())
  }
}