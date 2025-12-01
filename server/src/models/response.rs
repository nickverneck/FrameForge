//! Response data models for the FrameForge API
//!
//! This module defines the data transfer objects (DTOs) used for outgoing API responses.
//! The models are designed to match the Python FastAPI backend's response structure.

use serde::{Deserialize, Serialize};

/// Health check response
///
/// Returned by the `/api/health` endpoint to indicate the server is running.
///
/// # Example JSON Response
///
/// ```json
/// {
///   "status": "ok"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthResponse {
    /// Status indicator - always "ok" when the server is healthy
    pub status: String,
}

impl HealthResponse {
    /// Create a new healthy response
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self::ok()
    }
}

/// Providers list response
///
/// Returned by the `/api/providers` endpoint to list available AI providers.
///
/// # Example JSON Response
///
/// ```json
/// ["google", "nano-banana"]
/// ```
///
/// Note: This is just a Vec<String>, no wrapper object needed to match Python backend.
pub type ProvidersResponse = Vec<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_ok() {
        let response = HealthResponse::ok();
        assert_eq!(response.status, "ok");
    }

    #[test]
    fn test_health_response_default() {
        let response = HealthResponse::default();
        assert_eq!(response.status, "ok");
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse::ok();
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, r#"{"status":"ok"}"#);
    }

    #[test]
    fn test_providers_response_serialization() {
        let providers: ProvidersResponse = vec!["google".to_string(), "nano-banana".to_string()];
        let json = serde_json::to_string(&providers).unwrap();
        assert_eq!(json, r#"["google","nano-banana"]"#);
    }
}
