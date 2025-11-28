//! Request data models for the FrameForge API
//!
//! This module defines the data transfer objects (DTOs) used for incoming API requests.
//! The models are designed to match the Python FastAPI backend's request structure.

use serde::{Deserialize, Serialize};

/// Request structure for the `/api/edit` endpoint
///
/// This struct represents the multipart form data sent to the image editing endpoint.
/// It includes uploaded images, an optional text prompt for the AI, and optional provider selection.
///
/// # Fields
///
/// - `images`: List of uploaded image files (as bytes). Required field.
/// - `prompt`: Optional text prompt or style instructions for the AI.
///   If not provided, a default prompt will be used.
/// - `provider`: Optional provider selection (e.g., "google", "fal:fal-ai/flux/dev").
///   Defaults to "google" if not specified.
///
/// # Example Default Prompt
///
/// If no prompt is provided, the default is:
/// "Stage this room with minimalist modern furniture in neutral tones.
///  Preserve architecture and lighting; add realistic shadows and reflections."
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditImageRequest {
    /// Uploaded image files (required)
    /// In the actual multipart implementation, this will be handled by axum's multipart extractor
    #[serde(skip)]
    pub images: Vec<Vec<u8>>,

    /// Text prompt or style instructions (optional)
    /// If None, a default prompt will be used
    pub prompt: Option<String>,

    /// Provider selection (optional)
    /// Examples: "google", "nano-banana", "fal:fal-ai/flux/dev"
    /// Defaults to "google" if not specified
    pub provider: Option<String>,
}

impl EditImageRequest {
    /// Creates a new EditImageRequest with images and default values
    pub fn new(images: Vec<Vec<u8>>) -> Self {
        Self {
            images,
            prompt: None,
            provider: None,
        }
    }

    /// Creates a new EditImageRequest with all fields specified
    pub fn with_options(
        images: Vec<Vec<u8>>,
        prompt: Option<String>,
        provider: Option<String>,
    ) -> Self {
        Self {
            images,
            prompt,
            provider,
        }
    }

    /// Returns the default prompt used when no prompt is provided
    ///
    /// This matches the Python backend's default prompt exactly.
    pub fn default_prompt() -> &'static str {
        "Stage this room with minimalist modern furniture in neutral tones. \
         Preserve architecture and lighting; add realistic shadows and reflections."
    }

    /// Gets the prompt, using the default if none is specified
    pub fn get_prompt(&self) -> String {
        self.prompt
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| Self::default_prompt().to_string())
    }

    /// Gets the provider name, using the default if none is specified
    pub fn get_provider(&self) -> String {
        self.provider
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "google".to_string())
    }

    /// Validates the request
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// - No images are provided
    /// - Any image is empty
    pub fn validate(&self) -> Result<(), String> {
        if self.images.is_empty() {
            return Err("At least one image is required".to_string());
        }

        for (idx, img) in self.images.iter().enumerate() {
            if img.is_empty() {
                return Err(format!("Image {} is empty", idx));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_prompt() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        assert_eq!(
            request.get_prompt(),
            "Stage this room with minimalist modern furniture in neutral tones. \
             Preserve architecture and lighting; add realistic shadows and reflections."
        );
    }

    #[test]
    fn test_custom_prompt() {
        let request = EditImageRequest::with_options(
            vec![vec![1, 2, 3]],
            Some("Custom prompt".to_string()),
            None,
        );
        assert_eq!(request.get_prompt(), "Custom prompt");
    }

    #[test]
    fn test_empty_prompt_uses_default() {
        let request = EditImageRequest::with_options(
            vec![vec![1, 2, 3]],
            Some("   ".to_string()), // Only whitespace
            None,
        );
        assert_eq!(request.get_prompt(), EditImageRequest::default_prompt());
    }

    #[test]
    fn test_default_provider() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        assert_eq!(request.get_provider(), "google");
    }

    #[test]
    fn test_custom_provider() {
        let request = EditImageRequest::with_options(
            vec![vec![1, 2, 3]],
            None,
            Some("fal:fal-ai/flux/dev".to_string()),
        );
        assert_eq!(request.get_provider(), "fal:fal-ai/flux/dev");
    }

    #[test]
    fn test_validation_success() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_validation_no_images() {
        let request = EditImageRequest::new(vec![]);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_validation_empty_image() {
        let request = EditImageRequest::new(vec![vec![], vec![1, 2, 3]]);
        assert!(request.validate().is_err());
    }
}
