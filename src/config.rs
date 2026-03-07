pub const API_BASE_URL: &str = "http://localhost:8080/";
pub type DynError = Box<dyn std::error::Error + Send + Sync>;
pub type Error = Box<dyn std::error::Error>;