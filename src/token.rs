use dirs;
use std::env;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

use crate::model::tokenResponse::TokenDetailsResponse;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub token: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub userName: Option<String>,
    pub isValid: Option<bool>
}

pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    pub fn new(app: &str) -> Self {
        let mut path = env::current_dir().unwrap();

        path.push(".config");
        path.push(app);

        fs::create_dir_all(&path).unwrap();

        path.push("token.json");

        Self { path }
    }

    fn load(&self) -> Config {
        if !self.path.exists() {
            return Config::default();
        }

        let content = fs::read_to_string(&self.path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

     fn save(&self, config: &Config) {
        let content = serde_json::to_string_pretty(config).unwrap();
        fs::write(&self.path, content).unwrap();
    }

    pub fn setToken(&self, token: &str, tokenResponse: TokenDetailsResponse) {
        let mut cfg = self.load();
        cfg.token = Some(token.to_string());
        cfg.expiry = Some(tokenResponse.expiry);
        cfg.userName = Some(tokenResponse.userName);
        cfg.isValid = Some(tokenResponse.isValid);
        self.save(&cfg);
    }

    pub fn getToken(&self) -> Option<String> {
        self.load().token
    }

    pub fn deleteToken(&self) {
        let mut cfg = self.load();
        cfg.token = None;
        self.save(&cfg);
    }

    
    pub fn isBeforeExpiry(&self) -> bool {
        let now = Utc::now();
        match self.load().expiry {
            Some(expiryTime) => now < expiryTime,
            None => false,
        }
    }
    
    pub fn isTokenValid(&self) -> bool {
        let isTokenValid = self.load().isValid.unwrap_or(false);

        return isTokenValid && self.isBeforeExpiry();
    }
}


pub static CONFIG: Lazy<ConfigManager> =
    Lazy::new(|| ConfigManager::new("hls-cli"));