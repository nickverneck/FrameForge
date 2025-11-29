//! Fal.ai image editing service implementation
//!
//! This module provides integration with Fal.ai's image generation and editing models.
//! It supports dynamic model selection via the `fal:` prefix (e.g., `fal:fal-ai/flux/dev`).
//!
//! # Architecture
//!
//! The Fal.ai workflow consists of several steps:
//! 1. **Upload**: Convert images to base64 data URIs (no separate upload needed)
//! 2. **Submit**: POST request to the model endpoint with image data and prompt
//! 3. **Poll**: Use fal-client's subscribe mechanism which handles polling automatically
//! 4. **Download**: Fetch the result image from the returned URL or decode data URI
//!
//! # Example
//!
//! ```rust,no_run
//! use frameforge_server::services::fal_editor::FalEditor;
//! use frameforge_server::config::AppConfig;
//! use bytes::Bytes;
//!
//! async fn edit_with_fal(config: &AppConfig, image: Bytes, prompt: &str) -> anyhow::Result<Bytes> {
//!     let editor = FalEditor::new("fal-ai/flux/dev".to_string(), config)?;
//!     editor.edit_image(image, prompt).await
//! }
//! ```

use crate::config::AppConfig;
use crate::services::base::ImageEditor;
use anyhow::{anyhow, Context, Result};
use base64::Engine;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Fal.ai image editor implementation
///
/// This struct provides image editing functionality using Fal.ai's API.
/// It supports multiple Fal.ai models through the model_path parameter.
///
/// # Supported Models
///
/// - `fal-ai/nano-banana/edit` - Nano Banana editing model
/// - `fal-ai/qwen-image-edit` - Qwen image editing
/// - `fal-ai/bytedance/seedream/v4/edit` - Seedream v4
/// - `fal-ai/flux-kontext/dev` - Flux Kontext development model
/// - Any other Fal.ai model endpoint
pub struct FalEditor {
    /// The Fal.ai model path (e.g., "fal-ai/flux/dev")
    model_path: String,
    /// API key for Fal.ai authentication
    api_key: String,
    /// HTTP client for making requests
    client: reqwest::Client,
}

/// Request payload for Fal.ai image editing
#[derive(Debug, Serialize)]
struct FalRequest {
    /// Text prompt for image editing
    prompt: String,
    /// Image URL(s) or data URI for single image models
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    /// Image URLs for multi-image models
    #[serde(skip_serializing_if = "Option::is_none")]
    image_urls: Option<Vec<String>>,
    /// Output format (png, jpeg)
    output_format: String,
    /// Synchronous mode (returns result directly when complete)
    sync_mode: bool,
}

/// Response from Fal.ai API
#[derive(Debug, Deserialize)]
struct FalResponse {
    /// Single image result (some models)
    #[serde(default)]
    image: Option<FalImage>,
    /// Multiple images result (some models)
    #[serde(default)]
    images: Option<Vec<FalImage>>,
    /// Result wrapper (some models)
    #[serde(default)]
    result: Option<FalImage>,
}

/// Image data from Fal.ai response
#[derive(Debug, Deserialize)]
struct FalImage {
    /// URL or data URI of the generated image
    url: String,
}

impl FalEditor {
    /// Create a new Fal.ai editor instance
    ///
    /// # Arguments
    ///
    /// * `model_path` - The Fal.ai model path (e.g., "fal-ai/flux/dev")
    /// * `config` - Application configuration containing API keys
    ///
    /// # Returns
    ///
    /// Returns `Ok(FalEditor)` if an API key is available, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error if no FAL_KEY is configured in the environment or config.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use frameforge_server::services::fal_editor::FalEditor;
    /// use frameforge_server::config::AppConfig;
    ///
    /// fn create_editor(config: &AppConfig) -> anyhow::Result<FalEditor> {
    ///     FalEditor::new("fal-ai/flux/dev".to_string(), config)
    /// }
    /// ```
    pub fn new(model_path: String, config: &AppConfig) -> Result<Self> {
        let api_key = config
            .fal_key
            .as_ref()
            .ok_or_else(|| anyhow!("FAL_KEY not configured"))?
            .clone();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minutes for long-running generations
            .build()
            .context("Failed to create HTTP client")?;

        tracing::info!(
            model_path = %model_path,
            "Initialized Fal.ai editor"
        );

        Ok(Self {
            model_path,
            api_key,
            client,
        })
    }

    /// Determine the MIME type from image bytes
    ///
    /// Inspects the magic bytes at the start of the image data to determine format.
    fn detect_mime_type(bytes: &[u8]) -> &'static str {
        if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            "image/png"
        } else if bytes.starts_with(b"\xff\xd8\xff") {
            "image/jpeg"
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            "image/gif"
        } else if bytes.starts_with(b"RIFF") && bytes.len() > 12 && &bytes[8..12] == b"WEBP" {
            "image/webp"
        } else {
            "image/jpeg" // default fallback
        }
    }

    /// Convert image bytes to a base64 data URI
    ///
    /// This creates a self-contained data URI that can be sent directly to Fal.ai
    /// without requiring a separate upload step.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - Raw image data
    ///
    /// # Returns
    ///
    /// Returns a data URI string in the format: `data:{mime};base64,{base64_data}`
    fn bytes_to_data_uri(image_bytes: &Bytes) -> String {
        let mime = Self::detect_mime_type(image_bytes);
        let base64_data = base64::engine::general_purpose::STANDARD.encode(image_bytes);
        format!("data:{};base64,{}", mime, base64_data)
    }

    /// Submit an image editing request to Fal.ai
    ///
    /// This method handles the complete workflow:
    /// 1. Converts image to data URI
    /// 2. Submits to the model endpoint with sync_mode=true
    /// 3. Returns the result when complete
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The input image data
    /// * `prompt` - Text prompt describing desired edits
    ///
    /// # Returns
    ///
    /// Returns the result from Fal.ai, which may contain image URLs or data URIs
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error status
    /// - The response cannot be parsed
    async fn submit_request(&self, image_bytes: &Bytes, prompt: &str) -> Result<FalResponse> {
        // Convert image to data URI
        let data_uri = Self::bytes_to_data_uri(image_bytes);

        // Different models use different parameter names
        let use_single_image = self.model_path.contains("flux-kontext")
            || self.model_path.contains("qwen-image-edit");

        let request_body = if use_single_image {
            FalRequest {
                prompt: prompt.to_string(),
                image_url: Some(data_uri),
                image_urls: None,
                output_format: "png".to_string(),
                sync_mode: true,
            }
        } else {
            FalRequest {
                prompt: prompt.to_string(),
                image_url: None,
                image_urls: Some(vec![data_uri]),
                output_format: "png".to_string(),
                sync_mode: true,
            }
        };

        // Fal.ai uses a subscribe endpoint that handles polling automatically when sync_mode is true
        let url = format!("https://queue.fal.run/{}/subscribe", self.model_path);

        tracing::debug!(
            url = %url,
            model = %self.model_path,
            sync_mode = request_body.sync_mode,
            "Submitting request to Fal.ai"
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Key {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to Fal.ai")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(anyhow!(
                "Fal.ai API returned error {}: {}",
                status,
                error_text
            ));
        }

        let result: FalResponse = response
            .json()
            .await
            .context("Failed to parse Fal.ai response")?;

        tracing::debug!("Received response from Fal.ai");

        Ok(result)
    }

    /// Download an image from a URL
    ///
    /// Fetches the image data from an HTTP/HTTPS URL and returns it as bytes.
    ///
    /// # Arguments
    ///
    /// * `url` - The HTTP(S) URL of the image
    ///
    /// # Returns
    ///
    /// Returns a tuple of (image_bytes, mime_type)
    ///
    /// # Errors
    ///
    /// Returns an error if the download fails or the response is invalid
    async fn download_image(&self, url: &str) -> Result<(Bytes, Option<String>)> {
        tracing::debug!(url = %url, "Downloading image from URL");

        let response = self
            .client
            .get(url)
            .timeout(Duration::from_secs(120))
            .send()
            .await
            .context("Failed to download image from Fal.ai URL")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download image: HTTP {}",
                response.status()
            ));
        }

        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let bytes = response
            .bytes()
            .await
            .context("Failed to read image bytes")?;

        tracing::debug!(
            size = bytes.len(),
            mime_type = ?mime_type,
            "Successfully downloaded image"
        );

        Ok((bytes, mime_type))
    }

    /// Decode a base64 data URI into raw bytes
    ///
    /// # Arguments
    ///
    /// * `data_uri` - A data URI string (e.g., "data:image/png;base64,...")
    ///
    /// # Returns
    ///
    /// Returns a tuple of (image_bytes, mime_type)
    ///
    /// # Errors
    ///
    /// Returns an error if the data URI is malformed or base64 decoding fails
    fn decode_data_uri(data_uri: &str) -> Result<(Bytes, Option<String>)> {
        if !data_uri.starts_with("data:") {
            return Err(anyhow!("Not a data URI"));
        }

        let parts: Vec<&str> = data_uri.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Malformed data URI: missing comma separator"));
        }

        let header = parts[0];
        let base64_data = parts[1];

        // Extract MIME type from header (e.g., "data:image/png;base64")
        let mime_type = if header.starts_with("data:") && header.contains(';') {
            let mime = &header[5..header.find(';').unwrap_or(header.len())];
            Some(mime.to_string())
        } else {
            None
        };

        // Decode base64 data
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(base64_data)
            .context("Failed to decode base64 data from data URI")?;

        tracing::debug!(
            size = decoded.len(),
            mime_type = ?mime_type,
            "Successfully decoded data URI"
        );

        Ok((Bytes::from(decoded), mime_type))
    }

    /// Extract the image URL from a Fal.ai response
    ///
    /// Fal.ai responses can have different structures depending on the model.
    /// This method attempts to find the image URL in various response fields.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from Fal.ai
    ///
    /// # Returns
    ///
    /// Returns the URL if found, otherwise None
    fn extract_image_url(response: &FalResponse) -> Option<String> {
        // Try images array first
        if let Some(images) = &response.images {
            if let Some(first_image) = images.first() {
                return Some(first_image.url.clone());
            }
        }

        // Try single image field
        if let Some(image) = &response.image {
            return Some(image.url.clone());
        }

        // Try result field
        if let Some(result) = &response.result {
            return Some(result.url.clone());
        }

        None
    }
}

#[async_trait::async_trait]
impl ImageEditor for FalEditor {
    /// Edit an image using Fal.ai models
    ///
    /// This method implements the complete Fal.ai workflow:
    /// 1. Converts image to base64 data URI
    /// 2. Submits to Fal.ai with sync_mode=true (handles polling automatically)
    /// 3. Extracts result URL from response
    /// 4. Downloads or decodes the result image
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The input image data
    /// * `prompt` - Text description of desired edits
    ///
    /// # Returns
    ///
    /// Returns the edited image as bytes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The image cannot be converted to a data URI
    /// - The Fal.ai API request fails
    /// - No image URL is found in the response
    /// - The result image cannot be downloaded or decoded
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use frameforge_server::services::base::ImageEditor;
    /// use frameforge_server::services::fal_editor::FalEditor;
    /// use frameforge_server::config::AppConfig;
    /// use bytes::Bytes;
    ///
    /// async fn edit(config: &AppConfig, image: Bytes) -> anyhow::Result<Bytes> {
    ///     let editor = FalEditor::new("fal-ai/flux/dev".to_string(), config)?;
    ///     let prompt = "Add modern furniture to this room";
    ///     editor.edit_image(image, prompt).await
    /// }
    /// ```
    async fn edit_image(&self, image_bytes: Bytes, prompt: &str) -> Result<Bytes> {
        tracing::info!(
            model = %self.model_path,
            prompt = %prompt,
            image_size = image_bytes.len(),
            "Starting Fal.ai image editing"
        );

        // Submit request to Fal.ai (sync_mode handles polling automatically)
        let response = self
            .submit_request(&image_bytes, prompt)
            .await
            .context("Failed to submit request to Fal.ai")?;

        // Extract image URL from response
        let image_url = Self::extract_image_url(&response)
            .ok_or_else(|| anyhow!("No image URL found in Fal.ai response"))?;

        tracing::debug!(url = %image_url, "Got image URL from Fal.ai");

        // Handle different URL types
        let (result_bytes, _mime_type) = if image_url.starts_with("data:") {
            // Data URI - decode locally
            Self::decode_data_uri(&image_url)
                .context("Failed to decode data URI from Fal.ai")?
        } else {
            // HTTP(S) URL - download
            self.download_image(&image_url)
                .await
                .context("Failed to download result image")?
        };

        tracing::info!(
            result_size = result_bytes.len(),
            "Successfully completed Fal.ai image editing"
        );

        Ok(result_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_mime_type_png() {
        let png_header = b"\x89PNG\r\n\x1a\n";
        assert_eq!(FalEditor::detect_mime_type(png_header), "image/png");
    }

    #[test]
    fn test_detect_mime_type_jpeg() {
        let jpeg_header = b"\xff\xd8\xff";
        assert_eq!(FalEditor::detect_mime_type(jpeg_header), "image/jpeg");
    }

    #[test]
    fn test_detect_mime_type_gif() {
        let gif_header = b"GIF89a";
        assert_eq!(FalEditor::detect_mime_type(gif_header), "image/gif");
    }

    #[test]
    fn test_bytes_to_data_uri() {
        let image_data = Bytes::from_static(b"\x89PNG\r\n\x1a\ntest data");
        let data_uri = FalEditor::bytes_to_data_uri(&image_data);
        assert!(data_uri.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_decode_data_uri() {
        let test_data = b"Hello, World!";
        let base64_data = base64::engine::general_purpose::STANDARD.encode(test_data);
        let data_uri = format!("data:text/plain;base64,{}", base64_data);

        let result = FalEditor::decode_data_uri(&data_uri);
        assert!(result.is_ok());

        let (decoded, mime) = result.unwrap();
        assert_eq!(&decoded[..], test_data);
        assert_eq!(mime, Some("text/plain".to_string()));
    }

    #[test]
    fn test_decode_invalid_data_uri() {
        assert!(FalEditor::decode_data_uri("not a data uri").is_err());
        assert!(FalEditor::decode_data_uri("data:text/plain").is_err());
    }
}
