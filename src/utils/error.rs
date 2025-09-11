use thiserror::Error;

#[derive(Error, Debug)]
pub enum FplBotError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Discord error: {0}")]
    DiscordError(#[from] serenity::Error),
    
    #[error("Environment variable error: {0}")]
    EnvError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}