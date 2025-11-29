//! Configuration management module
//!
//! This module handles loading and validating application configuration from
//! environment variables and .env files. It uses dotenvy for .env file support
//! and the config crate for flexible configuration sources.

use serde::Deserialize;
use std::env;

/// Main application configuration structure
///
/// This struct holds all configuration values needed to run the server.
/// Values are loaded from environment variables, typically from a .env file.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Google API key for Gemini models
    pub google_api_key: Option<String>,

    /// Alternative Google API key (for backwards compatibility)
    pub gemini_api_key: Option<String>,

    /// Fal.ai API key for image generation models
    pub fal_key: Option<String>,

    /// Google model ID to use (e.g., "gemini-2.5-flash-image-preview")
    pub google_model_id: String,

    /// List of allowed CORS origins
    pub allowed_origins: Vec<String>,

    /// Server host address to bind to
    pub host: String,

    /// Server port to listen on
    pub port: u16,
}

impl AppConfig {
    /// Load configuration from environment variables
    ///
    /// This method loads the .env file if present, then reads configuration
    /// from environment variables with sensible defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration is invalid or missing
    pub fn load() -> anyhow::Result<Self> {
        // Load .env file if it exists (ignore errors if file doesn't exist)
        let _ = dotenvy::dotenv();

        // Load configuration values with defaults
        let google_api_key = env::var("GOOGLE_API_KEY").ok();
        let gemini_api_key = env::var("GEMINI_API_KEY").ok();
        let fal_key = env::var("FAL_KEY").ok();

        let google_model_id = env::var("GOOGLE_MODEL_ID")
            .unwrap_or_else(|_| "gemini-2.5-flash-image-preview".to_string());

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "*".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .unwrap_or(8000);

        let config = AppConfig {
            google_api_key,
            gemini_api_key,
            fal_key,
            google_model_id,
            allowed_origins,
            host,
            port,
        };

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate configuration values
    ///
    /// Checks that at least one API key is configured (or we're in development mode).
    /// Logs warnings for potentially insecure configurations.
    fn validate(&self) -> anyhow::Result<()> {
        // Warn if no API keys are configured
        if self.google_api_key.is_none()
            && self.gemini_api_key.is_none()
            && self.fal_key.is_none() {
            tracing::warn!(
                "No API keys configured. Server will run but AI providers will not be available."
            );
        }

        // Warn if using wildcard CORS in production-like setup
        if self.allowed_origins.contains(&"*".to_string()) && self.host != "127.0.0.1" {
            tracing::warn!(
                "CORS is configured with wildcard (*) on non-localhost host. \
                This is insecure for production. Set ALLOWED_ORIGINS explicitly."
            );
        }

        Ok(())
    }

    /// Get the effective Google API key
    ///
    /// Returns GOOGLE_API_KEY if set, otherwise falls back to GEMINI_API_KEY
    pub fn get_google_api_key(&self) -> Option<&str> {
        self.google_api_key
            .as_deref()
            .or(self.gemini_api_key.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_google_api_key_priority() {
        let config = AppConfig {
            google_api_key: Some("key1".to_string()),
            gemini_api_key: Some("key2".to_string()),
            fal_key: None,
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "0.0.0.0".to_string(),
            port: 8000,
        };

        assert_eq!(config.get_google_api_key(), Some("key1"));
    }

    #[test]
    fn test_get_google_api_key_fallback() {
        let config = AppConfig {
            google_api_key: None,
            gemini_api_key: Some("key2".to_string()),
            fal_key: None,
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "0.0.0.0".to_string(),
            port: 8000,
        };

        assert_eq!(config.get_google_api_key(), Some("key2"));
    }
}
