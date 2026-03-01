use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("HTTP error: {status} - {message}")]
    HttpError { 
        status: StatusCode, 
        message: String 
    },
}
