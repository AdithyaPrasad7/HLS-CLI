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
    pub userName: Option<String>
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

    pub fn set_token(&self, token: &str, tokenResponse: TokenDetailsResponse) {
        let mut cfg = self.load();
        cfg.token = Some(token.to_string());
        cfg.expiry = Some(tokenResponse.expiry);
        cfg.userName = Some(tokenResponse.userName);
        self.save(&cfg);
    }

    pub fn get_token(&self) -> Option<String> {
        self.load().token
    }

    pub fn delete_token(&self) {
        let mut cfg = self.load();
        cfg.token = None;
        self.save(&cfg);
    }
}


pub static CONFIG: Lazy<ConfigManager> =
    Lazy::new(|| ConfigManager::new("hls-cli"));