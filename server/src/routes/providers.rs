//! Providers listing endpoint
//!
//! This module implements the `/api/providers` endpoint for listing available AI providers.
//! The endpoint returns all statically configured providers based on available API keys.

use axum::{extract::State, Json};
use crate::config::AppConfig;
use crate::models::response::ProvidersResponse;
use crate::services::factory;

/// List available providers handler
///
/// Returns a JSON array of available AI image editing provider names.
/// The list is dynamically generated based on which API keys are configured.
///
/// # Endpoint
///
/// `GET /api/providers`
///
/// # Response
///
/// Returns a JSON array of provider names:
///
/// ```json
/// ["google", "nano-banana"]
/// ```
///
/// # Providers
///
/// ## Static Providers
/// - `"google"` - Google Gemini (requires GOOGLE_API_KEY or GEMINI_API_KEY)
/// - `"nano-banana"` - Alias for Google Gemini
///
/// ## Dynamic Providers
/// Dynamic `fal:*` providers are NOT listed here. They can be used at runtime
/// by specifying the model path (e.g., `"fal:fal-ai/flux/dev"`).
///
/// # Example
///
/// ```bash
/// curl http://localhost:8000/api/providers
/// ```
///
/// # State
///
/// Requires AppConfig to be in Axum shared state to check which API keys are configured.
pub async fn list_providers(
    State(config): State<AppConfig>,
) -> Json<ProvidersResponse> {
    let providers = factory::list_providers(&config);

    tracing::debug!(
        providers = ?providers,
        "Listing available providers"
    );

    Json(providers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_config() -> AppConfig {
        AppConfig {
            google_api_key: Some("test-key".to_string()),
            gemini_api_key: None,
            fal_key: Some("test-fal-key".to_string()),
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    #[tokio::test]
    async fn test_list_providers() {
        let config = make_test_config();
        let response = list_providers(State(config)).await;

        // Should include Google providers
        assert!(response.0.contains(&"google".to_string()));
        assert!(response.0.contains(&"nano-banana".to_string()));
    }

    #[tokio::test]
    async fn test_list_providers_no_keys() {
        let config = AppConfig {
            google_api_key: None,
            gemini_api_key: None,
            fal_key: None,
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "127.0.0.1".to_string(),
            port: 8000,
        };

        let response = list_providers(State(config)).await;

        // Should be empty when no keys configured
        assert!(response.0.is_empty());
    }
}
