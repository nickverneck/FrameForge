//! Google Gemini Flash image editing service (Nano Banana)
//!
//! This module implements the ImageEditor trait for Google's Gemini Flash model,
//! which provides image-to-image generation capabilities. The service uses the
//! genai crate for API communication and supports streaming responses.
//!
//! # Features
//!
//! - Streaming image generation using Gemini Flash
//! - Automatic MIME type detection
//! - Development mode fallback (returns original image if no API key)
//! - Base64 image encoding for API transmission
//!
//! # Configuration
//!
//! Requires either GOOGLE_API_KEY or GEMINI_API_KEY environment variable.
//! The model ID defaults to "gemini-2.5-flash-image-preview" but can be
//! configured via GOOGLE_MODEL_ID environment variable.
//!
//! # Implementation Notes
//!
//! This implementation uses the genai crate which is a multi-provider AI client.
//! Unlike the Python google-genai SDK which has specific support for image
//! generation with response_modalities, the genai crate uses a more generic
//! chat-based interface. The implementation:
//!
//! - Sends images as base64-encoded binary content parts
//! - Processes streaming responses looking for binary (image) content
//! - Currently supports single image input (per the ImageEditor trait)
//! - Extracts base64-encoded images from the response stream

use crate::config::AppConfig;
use crate::services::base::ImageEditor;
use anyhow::{anyhow, Context, Result};
use base64::Engine;
use bytes::Bytes;
use futures::StreamExt;
use genai::chat::{ChatMessage, ChatRequest, ContentPart, MessageContent};
use genai::Client;

/// Google Gemini Flash image editor implementation
///
/// This service uses Google's Gemini Flash model to perform image editing
/// operations based on text prompts. It supports streaming responses and
/// can handle multiple input images.
///
/// # Development Mode
///
/// If no API key is configured, the service falls back to returning the
/// original image unchanged. This allows development and testing without
/// requiring API credentials.
pub struct GoogleNanaBananaEditor {
    /// Google Gemini API client
    client: Option<Client>,
    /// Model ID to use for generation (e.g., "gemini-2.5-flash-image-preview")
    model_id: String,
    /// API key for authentication
    api_key: Option<String>,
}

impl GoogleNanaBananaEditor {
    /// Create a new Google Nano Banana editor instance
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration containing API keys and model settings
    ///
    /// # Returns
    ///
    /// Returns a new editor instance. If no API key is available, the editor
    /// will operate in development mode and return original images unchanged.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use frameforge_server::config::AppConfig;
    /// use frameforge_server::services::google_nano_banana::GoogleNanaBananaEditor;
    ///
    /// let config = AppConfig::load().unwrap();
    /// let editor = GoogleNanaBananaEditor::new(config);
    /// ```
    pub fn new(config: AppConfig) -> Self {
        let api_key = config.get_google_api_key().map(|s| s.to_string());
        let model_id = config.google_model_id.clone();

        // Initialize client only if we have an API key
        let client = api_key.as_ref().map(|_key| {
            // genai 0.5.0-alpha.2 gets API key from GOOGLE_API_KEY env var
            Client::default()
        });

        if api_key.is_none() {
            tracing::warn!(
                "Google provider initialized without API key. Will return original images unchanged."
            );
        }

        Self {
            client,
            model_id,
            api_key,
        }
    }

    /// Guess MIME type from raw image bytes
    ///
    /// This function inspects the magic bytes at the start of the image data
    /// to determine the image format.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw image bytes
    ///
    /// # Returns
    ///
    /// Returns a MIME type string, or "application/octet-stream" if unknown.
    fn guess_mime(data: &[u8]) -> &'static str {
        if data.is_empty() {
            return "application/octet-stream";
        }

        // JPEG magic bytes
        if data.len() >= 3 && data[0..3] == [0xFF, 0xD8, 0xFF] {
            return "image/jpeg";
        }

        // PNG magic bytes
        if data.len() >= 8 && data[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
            return "image/png";
        }

        // GIF magic bytes
        if data.len() >= 6 {
            if &data[0..6] == b"GIF87a" || &data[0..6] == b"GIF89a" {
                return "image/gif";
            }
        }

        // WebP magic bytes
        if data.len() >= 12
            && &data[0..4] == b"RIFF"
            && &data[8..12] == b"WEBP"
        {
            return "image/webp";
        }

        "application/octet-stream"
    }
}

#[async_trait::async_trait]
impl ImageEditor for GoogleNanaBananaEditor {
    /// Edit an image using Google Gemini Flash
    ///
    /// This method sends the input image(s) and prompt to the Gemini API,
    /// which returns an edited/generated image via streaming response.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The input image as raw bytes
    /// * `prompt` - Text description of the desired edits
    ///
    /// # Returns
    ///
    /// Returns the edited image as bytes, or the original image if in
    /// development mode (no API key configured).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails
    /// - No image is returned in the streaming response
    /// - The response cannot be parsed
    async fn edit_image(&self, image_bytes: Bytes, prompt: &str) -> Result<Bytes> {
        // Development mode fallback: no API key
        if self.api_key.is_none() || self.client.is_none() {
            tracing::warn!(
                "Google provider fallback: no API key found; returning original image."
            );
            return Ok(image_bytes);
        }

        let client = self.client.as_ref().unwrap();
        let model_id = self.model_id.clone();
        let prompt = prompt.to_string();
        let image_data = image_bytes.to_vec();

        // Detect input MIME type
        let input_mime = Self::guess_mime(&image_data);

        // Convert image to base64 for API transmission
        let base64_data = base64::engine::general_purpose::STANDARD.encode(&image_data);

        // Build content parts: image (as base64 binary) + text prompt
        let mut parts = vec![
            ContentPart::from_binary_base64(input_mime, base64_data, None),
            ContentPart::from_text(&prompt),
        ];

        // Create user message with image and prompt
        let message = ChatMessage::user(MessageContent::from_parts(parts));

        // Build chat request
        let chat_request = ChatRequest::new(vec![message]);

        // Execute the chat stream request
        let stream_response = client
            .exec_chat_stream(&model_id, chat_request, None)
            .await
            .context("Failed to execute chat stream request")?;

        let mut stream = stream_response.stream;
        let mut last_image_bytes: Option<Vec<u8>> = None;
        let mut last_image_mime: Option<String> = None;

        // Process streaming response chunks
        // Note: ChatStream implements the Stream trait, so we can use next() via StreamExt
        while let Some(event_result) = stream.next().await {
            let event = event_result.context("Error reading stream event")?;

            // We're looking for binary content in the stream events
            // The genai crate's ChatStreamEvent may contain content in different forms
            match event {
                genai::chat::ChatStreamEvent::Chunk(chunk) => {
                    // Text chunks don't contain image data, skip
                    continue;
                }
                genai::chat::ChatStreamEvent::End(end) => {
                    // Check captured_content for binary data
                    if let Some(content) = end.captured_content {
                        for part in content.parts() {
                            if let Some(binary) = part.as_binary() {
                                // Extract base64 image data and decode it
                                if let genai::chat::BinarySource::Base64(ref base64_str) = binary.source {
                                    let decoded = base64::engine::general_purpose::STANDARD
                                        .decode(base64_str.as_ref())
                                        .context("Failed to decode base64 image data")?;
                                    last_image_bytes = Some(decoded);
                                    last_image_mime = Some(binary.content_type.clone());
                                }
                            }
                        }
                    }
                }
                _ => {
                    // Other event types (Start, ReasoningChunk, etc.) don't contain image data
                    continue;
                }
            }
        }

        // Ensure we received an image
        let image_bytes = last_image_bytes
            .ok_or_else(|| anyhow!("No edited image returned from Gemini stream"))?;

        Ok(Bytes::from(image_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_mime_jpeg() {
        let jpeg_bytes = vec![0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(GoogleNanaBananaEditor::guess_mime(&jpeg_bytes), "image/jpeg");
    }

    #[test]
    fn test_guess_mime_png() {
        let png_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(GoogleNanaBananaEditor::guess_mime(&png_bytes), "image/png");
    }

    #[test]
    fn test_guess_mime_gif() {
        let gif_bytes = b"GIF89a".to_vec();
        assert_eq!(GoogleNanaBananaEditor::guess_mime(&gif_bytes), "image/gif");
    }

    #[test]
    fn test_guess_mime_webp() {
        let mut webp_bytes = b"RIFF".to_vec();
        webp_bytes.extend_from_slice(&[0, 0, 0, 0]); // File size placeholder
        webp_bytes.extend_from_slice(b"WEBP");
        assert_eq!(GoogleNanaBananaEditor::guess_mime(&webp_bytes), "image/webp");
    }

    #[test]
    fn test_guess_mime_unknown() {
        let unknown_bytes = vec![0x00, 0x01, 0x02];
        assert_eq!(
            GoogleNanaBananaEditor::guess_mime(&unknown_bytes),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_guess_mime_empty() {
        let empty_bytes = vec![];
        assert_eq!(
            GoogleNanaBananaEditor::guess_mime(&empty_bytes),
            "application/octet-stream"
        );
    }
}
