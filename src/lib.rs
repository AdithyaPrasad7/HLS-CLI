use once_cell::sync::Lazy;
use std::sync::Arc;
mod token;

pub static TOKEN_MGR: Lazy<Arc<token::TokenManager>> = Lazy::new(|| {
    Arc::new(token::TokenManager::new("HLS-CLI", "default"))
});