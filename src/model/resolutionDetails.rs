use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(clap::ValueEnum, Clone, Copy, Debug, EnumIter)]
pub enum ResolutionDetails {
  #[clap(name = "720p")]
  P720,
  #[clap(name = "480p")]
  P480,
  #[clap(name = "360p")]
  P360,
  #[clap(name = "240p")]
  P240
}

impl ResolutionDetails {
  pub fn name(&self) -> &'static str {
    match self {
      ResolutionDetails::P720 => "720p",
      ResolutionDetails::P480 => "480p",
      ResolutionDetails::P360 => "360p",
      ResolutionDetails::P240 => "240p",
    }
  }
      
  pub fn height(&self) -> u32 {
    match self {
      ResolutionDetails::P720 => 720,
      ResolutionDetails::P480 => 480,
      ResolutionDetails::P360 => 360,
      ResolutionDetails::P240 => 240,
    }
  }

  pub fn width(&self) -> u32 {
    match self {
      ResolutionDetails::P720 => 1280,
      ResolutionDetails::P480 => 854,
      ResolutionDetails::P360 => 640,
      ResolutionDetails::P240 => 426
    }
  }

  pub fn bandwidth(&self) -> u32 {
    match self {
      ResolutionDetails::P720 => 2500000,
      ResolutionDetails::P480 => 1000000,
      ResolutionDetails::P360 => 600000,
      ResolutionDetails::P240 => 300000
    }
  }

  pub fn getAll() -> Vec<ResolutionDetails> {
    ResolutionDetails::iter().collect()
  }
}