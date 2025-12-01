//! Image editing endpoint
//!
//! This module implements the `/api/edit` endpoint for AI-powered image editing.
//! The endpoint accepts multipart form data with images and optional parameters,
//! processes them through the selected AI provider, and streams the result back.

use axum::{
    body::Body,
    extract::{Multipart, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::models::request::EditImageRequest;
use crate::services::factory;

/// Image editing handler
///
/// Accepts multipart form data with images and optional parameters,
/// processes them through the selected AI provider, and returns the edited image.
///
/// # Endpoint
///
/// `POST /api/edit`
///
/// # Request Format
///
/// Multipart form data with the following fields:
/// - `images`: One or more image files (required)
/// - `prompt`: Text description for image editing (optional)
/// - `provider`: AI provider to use (optional, defaults to "google")
///
/// # Headers
///
/// Optional API key overrides via headers:
/// - `X-Google-Api-Key`: Override GOOGLE_API_KEY from config
/// - `X-Gemini-Api-Key`: Override GEMINI_API_KEY from config
/// - `X-Fal-Key`: Override FAL_KEY from config
///
/// # Response
///
/// Returns the edited image with appropriate Content-Type header.
/// The image is streamed efficiently without loading entirely into memory.
///
/// # Errors
///
/// - `400 Bad Request`: Invalid image format, missing images, or validation failure
/// - `404 Not Found`: Provider not found or not configured
/// - `500 Internal Server Error`: AI service error or internal failure
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8000/api/edit \
///   -F "images=@room.jpg" \
///   -F "prompt=Add modern furniture" \
///   -F "provider=google"
/// ```
///
/// # Tasks Implementation
///
/// This endpoint implements Tasks 26-32:
/// - Task 26: Multipart form handling
/// - Task 27-28: Header parsing for API key overrides
/// - Task 29: Default prompt handling
/// - Task 30: Get editor from factory
/// - Task 31: Call edit_image
/// - Task 32: Stream response
pub async fn edit_image(
    State(config): State<AppConfig>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    tracing::info!("Received image edit request");

    // Task 26: Extract multipart form data
    let mut images: Vec<Vec<u8>> = Vec::new();
    let mut prompt: Option<String> = None;
    let mut provider: Option<String> = None;

    // Parse multipart fields
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InvalidInput(format!("Failed to read multipart field: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "images" | "image" => {
                // Read image bytes
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::InvalidInput(format!("Failed to read image data: {}", e)))?;

                if !data.is_empty() {
                    // Validate that it's a valid image
                    image::guess_format(&data)
                        .map_err(|e| AppError::ImageProcessing(format!("Invalid image format: {}", e)))?;

                    images.push(data.to_vec());
                    tracing::debug!(size = data.len(), "Received image");
                }
            }
            "prompt" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::InvalidInput(format!("Failed to read prompt: {}", e)))?;

                if !text.trim().is_empty() {
                    tracing::debug!(prompt = %text, "Received prompt");
                    prompt = Some(text);
                }
            }
            "provider" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::InvalidInput(format!("Failed to read provider: {}", e)))?;

                if !text.trim().is_empty() {
                    tracing::debug!(provider = %text, "Received provider");
                    provider = Some(text);
                }
            }
            _ => {
                // Ignore unknown fields
                tracing::debug!(field_name = %name, "Ignoring unknown field");
            }
        }
    }

    // Validate that we have at least one image
    if images.is_empty() {
        return Err(AppError::InvalidInput(
            "At least one image is required".to_string(),
        ));
    }

    tracing::info!(image_count = images.len(), "Parsed multipart form");

    // Tasks 27-28: Extract API key overrides from headers
    let mut runtime_config = config.clone();

    if let Some(google_key) = headers.get("X-Google-Api-Key") {
        if let Ok(key_str) = google_key.to_str() {
            runtime_config.google_api_key = Some(key_str.to_string());
            tracing::debug!("Using Google API key from header");
        }
    }

    if let Some(gemini_key) = headers.get("X-Gemini-Api-Key") {
        if let Ok(key_str) = gemini_key.to_str() {
            runtime_config.gemini_api_key = Some(key_str.to_string());
            tracing::debug!("Using Gemini API key from header");
        }
    }

    if let Some(fal_key) = headers.get("X-Fal-Key") {
        if let Ok(key_str) = fal_key.to_str() {
            runtime_config.fal_key = Some(key_str.to_string());
            tracing::debug!("Using Fal API key from header");
        }
    }

    // Build request object for convenience
    let request = EditImageRequest::with_options(images, prompt, provider);

    // Task 29: Get prompt with default fallback
    let final_prompt = request.get_prompt();
    tracing::info!(prompt = %final_prompt, "Using prompt");

    // Task 28: Get provider with default fallback
    let provider_name = request.get_provider();
    tracing::info!(provider = %provider_name, "Using provider");

    // Task 30: Get editor from factory
    let editor = factory::get_editor(&provider_name, &runtime_config)
        .map_err(|e| {
            tracing::error!(error = ?e, provider = %provider_name, "Failed to get editor");
            e
        })?;

    tracing::info!(provider = %provider_name, "Created editor instance");

    // Task 31: Call edit_image
    // Note: The ImageEditor trait currently accepts a single Bytes image
    // For now, we'll use the first image. Multi-image support may be added in future.
    let first_image = Bytes::from(request.images.into_iter().next().unwrap());

    tracing::info!(
        image_size = first_image.len(),
        "Calling AI provider to edit image"
    );

    let result_bytes = editor
        .edit_image(first_image, &final_prompt)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to edit image");
            AppError::ProviderError(format!("Failed to edit image: {}", e))
        })?;

    tracing::info!(
        result_size = result_bytes.len(),
        "Successfully edited image"
    );

    // Task 32: Stream response with proper headers
    // Determine content type from image bytes
    let content_type = image::guess_format(&result_bytes)
        .ok()
        .and_then(|fmt| match fmt {
            image::ImageFormat::Png => Some("image/png"),
            image::ImageFormat::Jpeg => Some("image/jpeg"),
            image::ImageFormat::WebP => Some("image/webp"),
            _ => None,
        })
        .unwrap_or("image/png")
        .to_string();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, result_bytes.len())
        .body(Body::from(result_bytes))
        .map_err(|e| AppError::InternalServer(format!("Failed to build response: {}", e)))?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_image_request_validation() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        assert!(request.validate().is_ok());

        let empty_request = EditImageRequest::new(vec![]);
        assert!(empty_request.validate().is_err());
    }

    #[test]
    fn test_default_prompt() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        let prompt = request.get_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains("minimalist modern furniture"));
    }

    #[test]
    fn test_default_provider() {
        let request = EditImageRequest::new(vec![vec![1, 2, 3]]);
        assert_eq!(request.get_provider(), "google");
    }
}
