//! Factory pattern for creating image editor instances
//!
//! This module implements the factory pattern to decouple image editor selection
//! from the specific implementation classes. It provides functions to list available
//! providers and instantiate the appropriate editor based on provider name.
//!
//! # Supported Providers
//!
//! ## Static Providers
//! - `"google"` - Google Gemini (Nano Banana) editor
//! - `"nano-banana"` - Alias for Google Gemini editor
//!
//! ## Dynamic Providers
//! - `"fal:*"` - Fal.ai models with dynamic model path
//!   - Example: `"fal:fal-ai/flux/dev"`
//!   - Example: `"fal:fal-ai/flux-pro"`
//!
//! # Default Provider
//!
//! If an unknown provider is requested, the factory defaults to the Google
//! Gemini editor to ensure graceful degradation.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use frameforge_server::services::factory::{get_editor, list_providers};
//!
//! // List all available providers
//! let providers = list_providers();
//! println!("Available providers: {:?}", providers);
//!
//! // Get a specific editor
//! let editor = get_editor("google")?;
//! ```

use super::base::ImageEditor;
use super::fal_editor::FalEditor;
use super::google_nano_banana::GoogleNanaBananaEditor;
use crate::config::AppConfig;
use crate::error::AppError;

/// List all statically available image editor providers
///
/// This function returns a sorted list of provider names that can be used
/// with the `get_editor()` function. The list is dynamically generated based
/// on which API keys are configured in the provided config.
///
/// Note that dynamic `fal:*` providers are NOT listed here - any valid
/// Fal.ai model path can be used with the `fal:` prefix at runtime.
/// This matches the Python backend behavior of only listing static providers.
///
/// # Arguments
///
/// * `config` - Application configuration to check for available API keys
///
/// # Returns
///
/// A vector of provider names including:
/// - `"google"` and `"nano-banana"` - If GOOGLE_API_KEY or GEMINI_API_KEY is configured
/// - Dynamic `fal:*` providers are NOT enumerated (use `fal:model-path` at runtime)
///
/// # Example
///
/// ```rust
/// use frameforge_server::services::factory::list_providers;
/// use frameforge_server::config::AppConfig;
///
/// let config = AppConfig::load().unwrap();
/// let providers = list_providers(&config);
/// // Providers list depends on which API keys are configured
/// ```
pub fn list_providers(config: &AppConfig) -> Vec<String> {
    let mut providers = Vec::new();

    // Include Google providers if API key is available
    // Note: We only list static providers here, not dynamic fal:* providers
    if config.get_google_api_key().is_some() {
        providers.push("google".to_string());
        providers.push("nano-banana".to_string());
    }

    providers.sort();
    providers
}

/// Get an image editor instance for the specified provider
///
/// This factory function creates and returns an appropriate `ImageEditor` implementation
/// based on the provider name. It handles both static providers (google, nano-banana)
/// and dynamic fal: prefixed providers.
///
/// # Arguments
///
/// * `provider_name` - The name of the provider to use
///   - Static providers: "google", "nano-banana"
///   - Dynamic providers: "fal:model-path" (e.g., "fal:fal-ai/flux/dev")
/// * `config` - Application configuration containing API keys
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(Box<dyn ImageEditor>)` - A boxed trait object for the requested provider
/// - `Err(AppError)` - An error if the provider cannot be created or API key is missing
///
/// # Provider Parsing
///
/// ## Static Providers
/// For "google" and "nano-banana", the function will instantiate a Google Gemini editor.
/// Requires GOOGLE_API_KEY or GEMINI_API_KEY to be configured.
///
/// ## Dynamic Fal Providers
/// For providers prefixed with "fal:", the function extracts the model path:
/// - Input: "fal:fal-ai/flux/dev"
/// - Extracted model path: "fal-ai/flux/dev"
/// - Creates a FalEditor with the specified model
/// - Requires FAL_KEY to be configured
///
/// ## Unknown Providers
/// If a provider is not recognized, the function defaults to the Google Gemini editor
/// to ensure graceful degradation (if Google API key is available).
///
/// # Errors
///
/// Returns `AppError::ProviderNotFound` if:
/// - Invalid fal: format (empty model path)
/// - Required API key is not configured
/// - Unknown provider and no Google API key for fallback
///
/// # Examples
///
/// ```rust,no_run
/// use frameforge_server::services::factory::get_editor;
/// use frameforge_server::config::AppConfig;
///
/// let config = AppConfig::load().unwrap();
///
/// // Get Google Gemini editor
/// let google_editor = get_editor("google", &config)?;
///
/// // Get Fal.ai editor with specific model
/// let fal_editor = get_editor("fal:fal-ai/flux/dev", &config)?;
///
/// // Unknown provider defaults to Google (if available)
/// let default_editor = get_editor("unknown-provider", &config)?;
/// # Ok::<(), frameforge_server::error::AppError>(())
/// ```
pub fn get_editor(provider_name: &str, config: &AppConfig) -> Result<Box<dyn ImageEditor>, AppError> {
    // Normalize provider name: lowercase and trim whitespace (matches Python behavior)
    let normalized_name = provider_name.trim().to_lowercase();

    // Handle dynamic fal: providers
    if normalized_name.starts_with("fal:") {
        // Extract model path from "fal:model-path" format using normalized name
        let model_path = normalized_name
            .strip_prefix("fal:")
            .ok_or_else(|| {
                AppError::ProviderNotFound(format!(
                    "Invalid fal provider format: {}. Expected format: fal:model-path",
                    normalized_name
                ))
            })?
            .trim();

        // Validate model path is not empty
        if model_path.is_empty() {
            return Err(AppError::ProviderNotFound(
                "Fal provider requires a model path. Format: fal:model-path".to_string(),
            ));
        }

        // Check if FAL_KEY is configured
        if config.fal_key.is_none() {
            return Err(AppError::ProviderNotFound(
                "Fal provider requested but FAL_KEY is not configured in environment".to_string(),
            ));
        }

        // Create and return FalEditor
        let editor = FalEditor::new(model_path.to_string(), config)
            .map_err(|e| AppError::ProviderNotFound(format!("Failed to create Fal editor: {}", e)))?;

        tracing::info!(
            provider = provider_name,
            normalized = normalized_name,
            model_path = model_path,
            "Created Fal.ai editor"
        );

        return Ok(Box::new(editor));
    }

    // Handle static providers using normalized name
    match normalized_name.as_str() {
        "google" | "nano-banana" => {
            // Check if Google API key is configured
            if config.get_google_api_key().is_none() {
                return Err(AppError::ProviderNotFound(
                    "Google provider requested but GOOGLE_API_KEY/GEMINI_API_KEY is not configured in environment".to_string(),
                ));
            }

            // Create and return GoogleNanaBananaEditor
            let editor = GoogleNanaBananaEditor::new(config.clone());

            tracing::info!(
                provider = provider_name,
                model_id = %config.google_model_id,
                "Created Google Nano Banana editor"
            );

            Ok(Box::new(editor))
        }
        // Default to Google provider for unknown names (graceful degradation)
        _ => {
            tracing::warn!(
                provider = provider_name,
                "Unknown provider requested, defaulting to Google Gemini"
            );

            // Check if Google API key is configured for fallback
            if config.get_google_api_key().is_none() {
                return Err(AppError::ProviderNotFound(format!(
                    "Provider '{}' not found and cannot fallback to Google (no API key configured)",
                    provider_name
                )));
            }

            // Return GoogleNanaBananaEditor as default
            let editor = GoogleNanaBananaEditor::new(config.clone());

            tracing::info!(
                provider = provider_name,
                fallback = "google",
                model_id = %config.google_model_id,
                "Defaulting to Google Nano Banana editor"
            );

            Ok(Box::new(editor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_config() -> AppConfig {
        AppConfig {
            google_api_key: Some("test-google-key".to_string()),
            gemini_api_key: None,
            fal_key: Some("test-fal-key".to_string()),
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    fn make_config_no_keys() -> AppConfig {
        AppConfig {
            google_api_key: None,
            gemini_api_key: None,
            fal_key: None,
            google_model_id: "test-model".to_string(),
            allowed_origins: vec!["*".to_string()],
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    #[test]
    fn test_list_providers_with_all_keys() {
        let config = make_test_config();
        let providers = list_providers(&config);

        // Should include only Google providers (Fal is dynamic, not listed)
        assert!(providers.contains(&"google".to_string()));
        assert!(providers.contains(&"nano-banana".to_string()));
        // Dynamic fal: providers are not enumerated in the list
        assert!(!providers.iter().any(|p| p.starts_with("fal:")));
    }

    #[test]
    fn test_list_providers_no_keys() {
        let config = make_config_no_keys();
        let providers = list_providers(&config);

        // Should be empty when no keys configured
        assert!(providers.is_empty());
    }

    #[test]
    fn test_list_providers_only_google() {
        let mut config = make_config_no_keys();
        config.google_api_key = Some("test-key".to_string());
        let providers = list_providers(&config);

        // Should include only Google providers
        assert!(providers.contains(&"google".to_string()));
        assert!(providers.contains(&"nano-banana".to_string()));
        assert!(!providers.iter().any(|p| p.starts_with("fal:")));
    }

    #[test]
    fn test_list_providers_only_fal() {
        let mut config = make_config_no_keys();
        config.fal_key = Some("test-key".to_string());
        let providers = list_providers(&config);

        // Should NOT include Fal providers in list (they are dynamic, not static)
        // This matches Python backend behavior
        assert!(!providers.contains(&"google".to_string()));
        assert!(providers.is_empty()); // No static providers with only FAL_KEY
    }

    #[test]
    fn test_list_providers_sorted() {
        let config = make_test_config();
        let providers = list_providers(&config);
        let mut sorted = providers.clone();
        sorted.sort();
        assert_eq!(providers, sorted);
    }

    #[test]
    fn test_get_google_editor() {
        let config = make_test_config();
        let result = get_editor("google", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_nano_banana_editor() {
        let config = make_test_config();
        let result = get_editor("nano-banana", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_google_editor_no_key() {
        let config = make_config_no_keys();
        let result = get_editor("google", &config);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("not configured"));
        }
    }

    #[test]
    fn test_fal_provider_parsing() {
        let config = make_test_config();
        let result = get_editor("fal:fal-ai/flux/dev", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_fal_model_path() {
        let config = make_test_config();
        let result = get_editor("fal:", &config);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("requires a model path"));
        }
    }

    #[test]
    fn test_fal_provider_no_key() {
        let config = make_config_no_keys();
        let result = get_editor("fal:fal-ai/flux/dev", &config);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("not configured"));
        }
    }

    #[test]
    fn test_unknown_provider_defaults_to_google() {
        let config = make_test_config();
        let result = get_editor("unknown-provider", &config);
        // Should default to Google successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_unknown_provider_no_fallback() {
        let config = make_config_no_keys();
        let result = get_editor("unknown-provider", &config);
        // Should fail when no Google key available
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("not found"));
        }
    }

    #[test]
    fn test_provider_name_normalization_uppercase() {
        let config = make_test_config();
        // Test uppercase provider names are normalized
        assert!(get_editor("GOOGLE", &config).is_ok());
        assert!(get_editor("Nano-Banana", &config).is_ok());
        assert!(get_editor("FAL:fal-ai/flux/dev", &config).is_ok());
    }

    #[test]
    fn test_provider_name_normalization_whitespace() {
        let config = make_test_config();
        // Test whitespace is trimmed
        assert!(get_editor("  google  ", &config).is_ok());
        assert!(get_editor(" nano-banana ", &config).is_ok());
        assert!(get_editor(" fal:fal-ai/flux/dev ", &config).is_ok());
    }

    #[test]
    fn test_provider_name_normalization_mixed() {
        let config = make_test_config();
        // Test combined uppercase and whitespace
        assert!(get_editor("  GOOGLE  ", &config).is_ok());
        assert!(get_editor(" Nano-BANANA ", &config).is_ok());
        assert!(get_editor("  FAL:fal-ai/FLUX/dev  ", &config).is_ok());
    }
}
