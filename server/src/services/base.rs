//! Base trait for image editing services
//!
//! This module defines the core abstraction for image editing operations across
//! different AI providers. All provider implementations must implement the
//! `ImageEditor` trait to ensure a consistent interface.
//!
//! # Design Philosophy
//!
//! The `ImageEditor` trait provides a standardized interface that:
//! - Accepts raw image data as bytes for maximum flexibility
//! - Uses async methods for non-blocking I/O operations
//! - Returns edited images with optional MIME type information
//! - Handles errors uniformly across all providers
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use bytes::Bytes;
//! use frameforge_server::services::base::ImageEditor;
//!
//! async fn process_image(editor: &dyn ImageEditor, image: Bytes, prompt: &str) -> Result<Bytes, anyhow::Error> {
//!     editor.edit_image(image, prompt).await
//! }
//! ```

use bytes::Bytes;

/// Core trait for image editing services
///
/// This trait defines the interface that all AI image editing providers must implement.
/// It provides a single method for editing images based on text prompts.
///
/// # Thread Safety
///
/// Implementations must be both `Send` and `Sync` to support concurrent usage
/// across multiple async tasks in the Axum server.
///
/// # Error Handling
///
/// Implementations should use `anyhow::Error` for internal error handling,
/// which provides rich context and error chaining. The error will be mapped
/// to appropriate HTTP responses by the route handlers.
#[async_trait::async_trait]
pub trait ImageEditor: Send + Sync {
    /// Edit an image based on a text prompt
    ///
    /// This method takes an input image and a text prompt, sends them to an AI
    /// service for processing, and returns the edited image.
    ///
    /// # Arguments
    ///
    /// * `image_bytes` - The raw bytes of the input image (JPEG, PNG, etc.)
    /// * `prompt` - A text description of the desired edits or transformation
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(Bytes)` - The edited image as raw bytes
    /// - `Err(anyhow::Error)` - An error if the editing operation failed
    ///
    /// # Errors
    ///
    /// This method can fail for various reasons:
    /// - Invalid image format or corrupted image data
    /// - API authentication failures (missing or invalid API keys)
    /// - Network errors when communicating with AI services
    /// - AI service rate limits or quota exceeded
    /// - Timeout waiting for AI processing to complete
    /// - Invalid or unsupported prompts
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bytes::Bytes;
    /// use frameforge_server::services::base::ImageEditor;
    ///
    /// async fn edit_room_image(editor: &dyn ImageEditor, image: Bytes) -> anyhow::Result<Bytes> {
    ///     let prompt = "Add modern minimalist furniture to this room";
    ///     editor.edit_image(image, prompt).await
    /// }
    /// ```
    async fn edit_image(&self, image_bytes: Bytes, prompt: &str) -> Result<Bytes, anyhow::Error>;
}
