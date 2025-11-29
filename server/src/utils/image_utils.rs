//! Image processing utility functions
//!
//! This module provides utilities for working with images including:
//! - Validation of image format
//! - MIME type detection
//! - Base64 encoding/decoding
//! - Image format conversion
//!
//! All functions are designed to work with `bytes::Bytes` for efficient
//! zero-copy operations.

use crate::error::{AppError, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bytes::Bytes;
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

/// Validate that the provided bytes represent a valid image
///
/// This function attempts to load the image to verify it's in a valid format.
/// It supports all formats that the `image` crate can decode.
///
/// # Arguments
///
/// * `data` - The image bytes to validate
///
/// # Returns
///
/// * `Ok(())` if the bytes represent a valid image
/// * `Err(AppError)` if the bytes are not a valid image
///
/// # Example
///
/// ```no_run
/// use bytes::Bytes;
/// use frameforge_server::utils::image_utils::validate_image_bytes;
///
/// let image_data = Bytes::from(vec![/* image bytes */]);
/// validate_image_bytes(&image_data)?;
/// # Ok::<(), frameforge_server::error::AppError>(())
/// ```
pub fn validate_image_bytes(data: &[u8]) -> Result<()> {
    // Try to detect and decode the image format
    let reader = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| AppError::ImageProcessing(format!("Failed to read image format: {}", e)))?;

    // Verify we could detect a format
    if reader.format().is_none() {
        return Err(AppError::ImageProcessing(
            "Unable to detect image format. File may be corrupted or not an image.".to_string(),
        ));
    }

    // Try to decode the image to ensure it's valid
    reader
        .decode()
        .map_err(|e| AppError::ImageProcessing(format!("Failed to decode image: {}", e)))?;

    Ok(())
}

/// Load an image from bytes
///
/// This function decodes image bytes into an `image::DynamicImage` that can
/// be manipulated or converted to other formats.
///
/// # Arguments
///
/// * `data` - The image bytes to load
///
/// # Returns
///
/// * `Ok(DynamicImage)` containing the decoded image
/// * `Err(AppError)` if the image cannot be decoded
pub fn bytes_to_image(data: &[u8]) -> Result<image::DynamicImage> {
    let img = image::load_from_memory(data)
        .map_err(|e| AppError::ImageProcessing(format!("Failed to load image: {}", e)))?;
    Ok(img)
}

/// Convert an image to bytes in the specified format
///
/// This function encodes a `DynamicImage` into bytes using the specified format.
///
/// # Arguments
///
/// * `img` - The image to encode
/// * `format` - The desired output format (PNG, JPEG, etc.)
///
/// # Returns
///
/// * `Ok(Bytes)` containing the encoded image
/// * `Err(AppError)` if encoding fails
pub fn image_to_bytes(img: &image::DynamicImage, format: ImageFormat) -> Result<Bytes> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, format)
        .map_err(|e| AppError::ImageProcessing(format!("Failed to encode image: {}", e)))?;

    Ok(Bytes::from(buffer))
}

/// Convert image bytes to a base64-encoded data URL
///
/// This function creates a data URL suitable for embedding in HTML or sending
/// to APIs that expect base64-encoded images.
///
/// # Arguments
///
/// * `data` - The image bytes to encode
/// * `mime_type` - Optional MIME type. If None, it will be detected automatically.
///
/// # Returns
///
/// * `Ok(String)` containing the data URL (e.g., "data:image/png;base64,...")
/// * `Err(AppError)` if MIME type detection fails
///
/// # Example
///
/// ```no_run
/// use bytes::Bytes;
/// use frameforge_server::utils::image_utils::bytes_to_base64;
///
/// let image_data = Bytes::from(vec![/* image bytes */]);
/// let data_url = bytes_to_base64(&image_data, None)?;
/// // data_url will be like: "data:image/png;base64,iVBORw0KGgo..."
/// # Ok::<(), frameforge_server::error::AppError>(())
/// ```
pub fn bytes_to_base64(data: &[u8], mime_type: Option<&str>) -> Result<String> {
    let mime = match mime_type {
        Some(m) => m.to_string(),
        None => get_mime_type(data)?,
    };

    let encoded = STANDARD.encode(data);
    Ok(format!("data:{};base64,{}", mime, encoded))
}

/// Decode a base64-encoded data URL or base64 string
///
/// This function decodes base64 strings, optionally stripping data URL prefixes.
///
/// # Arguments
///
/// * `base64_str` - The base64 string or data URL to decode
///
/// # Returns
///
/// * `Ok(Bytes)` containing the decoded image data
/// * `Err(AppError)` if decoding fails
pub fn base64_to_bytes(base64_str: &str) -> Result<Bytes> {
    // Strip data URL prefix if present (e.g., "data:image/png;base64,")
    let base64_data = if let Some(comma_pos) = base64_str.find(',') {
        &base64_str[comma_pos + 1..]
    } else {
        base64_str
    };

    let decoded = STANDARD
        .decode(base64_data)
        .map_err(|e| AppError::ImageProcessing(format!("Failed to decode base64: {}", e)))?;

    Ok(Bytes::from(decoded))
}

/// Determine the MIME type of image data
///
/// This function examines the image bytes to determine the MIME type.
/// It uses the `image` crate's format detection.
///
/// # Arguments
///
/// * `data` - The image bytes to analyze
///
/// # Returns
///
/// * `Ok(String)` containing the MIME type (e.g., "image/png")
/// * `Err(AppError)` if the format cannot be determined
///
/// # Example
///
/// ```no_run
/// use bytes::Bytes;
/// use frameforge_server::utils::image_utils::get_mime_type;
///
/// let image_data = Bytes::from(vec![/* PNG image bytes */]);
/// let mime = get_mime_type(&image_data)?;
/// assert_eq!(mime, "image/png");
/// # Ok::<(), frameforge_server::error::AppError>(())
/// ```
pub fn get_mime_type(data: &[u8]) -> Result<String> {
    let reader = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| AppError::ImageProcessing(format!("Failed to detect image format: {}", e)))?;

    let format = reader.format().ok_or_else(|| {
        AppError::ImageProcessing(
            "Unable to determine image format. File may be corrupted or not an image.".to_string(),
        )
    })?;

    let mime = format_to_mime_type(format);
    Ok(mime.to_string())
}

/// Convert ImageFormat to MIME type string
///
/// # Arguments
///
/// * `format` - The image format
///
/// # Returns
///
/// The corresponding MIME type string
fn format_to_mime_type(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Ico => "image/x-icon",
        ImageFormat::Tiff => "image/tiff",
        ImageFormat::Avif => "image/avif",
        ImageFormat::Pnm => "image/x-portable-anymap",
        ImageFormat::Dds => "image/vnd-ms.dds",
        ImageFormat::Tga => "image/x-tga",
        ImageFormat::OpenExr => "image/x-exr",
        ImageFormat::Farbfeld => "image/x-farbfeld",
        ImageFormat::Hdr => "image/vnd.radiance",
        ImageFormat::Qoi => "image/qoi",
        // Default to octet-stream for unknown formats
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a minimal valid PNG image for testing
    fn create_test_png() -> Vec<u8> {
        // Minimal 1x1 white PNG
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE,
            0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk
            0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F, 0x00, 0x05, 0xFE, 0x02, 0xFE,
            0xDC, 0xCC, 0x59, 0xE7,
            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
            0xAE, 0x42, 0x60, 0x82,
        ]
    }

    #[test]
    fn test_validate_image_bytes_valid() {
        let png_data = create_test_png();
        assert!(validate_image_bytes(&png_data).is_ok());
    }

    #[test]
    fn test_validate_image_bytes_invalid() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(validate_image_bytes(&invalid_data).is_err());
    }

    #[test]
    fn test_get_mime_type() {
        let png_data = create_test_png();
        let mime = get_mime_type(&png_data).unwrap();
        assert_eq!(mime, "image/png");
    }

    #[test]
    fn test_bytes_to_base64() {
        let png_data = create_test_png();
        let data_url = bytes_to_base64(&png_data, Some("image/png")).unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_base64_roundtrip() {
        let png_data = create_test_png();
        let data_url = bytes_to_base64(&png_data, Some("image/png")).unwrap();
        let decoded = base64_to_bytes(&data_url).unwrap();
        assert_eq!(png_data, decoded.to_vec());
    }

    #[test]
    fn test_bytes_to_image_and_back() {
        let png_data = create_test_png();
        let img = bytes_to_image(&png_data).unwrap();
        let bytes = image_to_bytes(&img, ImageFormat::Png).unwrap();
        // The re-encoded image should still be valid
        assert!(validate_image_bytes(&bytes).is_ok());
    }

    #[test]
    fn test_format_to_mime_type() {
        assert_eq!(format_to_mime_type(ImageFormat::Png), "image/png");
        assert_eq!(format_to_mime_type(ImageFormat::Jpeg), "image/jpeg");
        assert_eq!(format_to_mime_type(ImageFormat::WebP), "image/webp");
    }
}
