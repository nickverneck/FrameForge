//! Error handling module for FrameForge server
//!
//! This module defines custom error types using thiserror for domain-specific errors
//! and provides Axum integration through IntoResponse implementation.
//!
//! # Error Strategy
//! - Use `AppError` with thiserror for public API boundaries
//! - Use `anyhow::Error` for internal provider implementation details
//! - Map each error variant to appropriate HTTP status codes
//! - Provide user-friendly error messages in JSON format

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Main application error type for API boundaries
///
/// This enum represents all possible errors that can occur in the FrameForge server.
/// Each variant maps to a specific HTTP status code and error message.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Configuration-related errors (missing API keys, invalid settings, etc.)
    #[error("Configuration error: {0}")]
    Config(String),

    /// Image processing errors (invalid format, corrupted data, etc.)
    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    /// Provider not found error
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    /// Generic provider error with context
    #[error("Provider error: {0}")]
    ProviderError(String),

    /// Invalid input from client (bad request data)
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Internal server error with context
    #[error("Internal server error: {0}")]
    InternalServer(String),

    /// Catch-all for anyhow errors from internal operations
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// JSON error response structure
///
/// This is the format that will be sent to clients when an error occurs.
#[derive(serde::Serialize)]
struct ErrorResponse {
    /// The error message
    error: String,
    /// Error type/code for programmatic handling
    #[serde(skip_serializing_if = "Option::is_none")]
    error_type: Option<String>,
}

impl AppError {
    /// Map error variant to HTTP status code
    fn status_code(&self) -> StatusCode {
        match self {
            // 400 Bad Request - client error
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::ImageProcessing(_) => StatusCode::BAD_REQUEST,

            // 404 Not Found - resource not found
            AppError::ProviderNotFound(_) => StatusCode::NOT_FOUND,

            // 500 Internal Server Error - server/provider errors
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ProviderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error type string for programmatic handling
    fn error_type(&self) -> &'static str {
        match self {
            AppError::Config(_) => "config_error",
            AppError::ImageProcessing(_) => "image_processing_error",
            AppError::ProviderNotFound(_) => "provider_not_found",
            AppError::ProviderError(_) => "provider_error",
            AppError::InvalidInput(_) => "invalid_input",
            AppError::InternalServer(_) => "internal_server_error",
            AppError::Internal(_) => "internal_error",
        }
    }
}

impl IntoResponse for AppError {
    /// Convert AppError into an Axum HTTP response
    ///
    /// This implementation ensures all errors are returned as JSON with appropriate
    /// HTTP status codes and user-friendly error messages.
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let error_message = self.to_string();
        let error_type = self.error_type().to_string();

        // Log the error with appropriate level
        match status_code {
            StatusCode::INTERNAL_SERVER_ERROR => {
                tracing::error!(
                    error = ?self,
                    status = ?status_code,
                    "Internal server error occurred"
                );
            }
            StatusCode::BAD_REQUEST => {
                tracing::warn!(
                    error = %error_message,
                    status = ?status_code,
                    "Client error occurred"
                );
            }
            _ => {
                tracing::info!(
                    error = %error_message,
                    status = ?status_code,
                    "Error occurred"
                );
            }
        }

        // Build JSON error response
        let body = Json(ErrorResponse {
            error: error_message,
            error_type: Some(error_type),
        });

        (status_code, body).into_response()
    }
}

// Convenience conversions from common error types

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::ImageProcessing(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalServer(format!("I/O error: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidInput(format!("JSON parsing error: {}", err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::ProviderError(format!("Request timeout: {}", err))
        } else if err.is_connect() {
            AppError::ProviderError(format!("Connection error: {}", err))
        } else if err.is_status() {
            AppError::ProviderError(format!("HTTP error: {}", err))
        } else {
            AppError::ProviderError(err.to_string())
        }
    }
}

/// Result type alias for operations that can fail with AppError
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            AppError::InvalidInput("test".into()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::ProviderNotFound("test".into()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::Config("test".into()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_types() {
        assert_eq!(
            AppError::InvalidInput("test".into()).error_type(),
            "invalid_input"
        );
        assert_eq!(
            AppError::ProviderNotFound("test".into()).error_type(),
            "provider_not_found"
        );
    }

    #[test]
    fn test_error_display() {
        let err = AppError::InvalidInput("bad data".into());
        assert_eq!(err.to_string(), "Invalid input: bad data");
    }
}
