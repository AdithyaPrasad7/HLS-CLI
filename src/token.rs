use keyring::{Entry, Credential, Result as KeyringResult};
use thiserror::Error;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
    #[error("Token not found")]
    TokenNotFound,
    #[error("Failed to parse token")]
    ParseError,
}

pub struct TokenManager {
    service: String,
    username: String,
}

impl TokenManager {
    pub fn new(service: &str, username: &str) -> Self {
        Self {
            service: service.to_string(),
            username: username.to_string(),
        }
    }

    pub fn set_token(&self, token: &str) -> KeyringResult<()> {
        let entry = Entry::new(&self.service, &self.username)?;
        entry.set_password(token)
    }

    pub fn get_token(&self) -> Result<Option<String>, TokenError> {
        let entry = Entry::new(&self.service, &self.username)
            .map_err(TokenError::Keyring)?;
        
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(TokenError::Keyring(e)),
        }
    }

    pub fn delete_token(&self) -> KeyringResult<()> {
        let entry = Entry::new(&self.service, &self.username)?;
        entry.delete_credential()
    }

    pub fn has_token(&self) -> Result<bool, TokenError> {
        Ok(self.get_token()?.is_some())
    }
}


pub static TOKEN_MGR: Lazy<Arc<TokenManager>> = Lazy::new(|| {
    Arc::new(TokenManager::new(env!("CARGO_PKG_NAME"), "token"))
});