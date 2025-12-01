//! Health check endpoint
//!
//! This module implements the `/api/health` endpoint for monitoring and health checks.
//! The endpoint provides a simple way to verify that the server is running and responsive.

use axum::Json;
use crate::models::response::HealthResponse;

/// Health check handler
///
/// Returns a simple JSON response indicating the server is healthy.
///
/// # Endpoint
///
/// `GET /api/health`
///
/// # Response
///
/// Returns a JSON object with status "ok":
///
/// ```json
/// {
///   "status": "ok"
/// }
/// ```
///
/// # Example
///
/// ```bash
/// curl http://localhost:8000/api/health
/// ```
///
/// This endpoint is typically used by:
/// - Load balancers for health checks
/// - Monitoring systems (e.g., Prometheus, Datadog)
/// - Container orchestration platforms (e.g., Kubernetes)
/// - CI/CD pipelines to verify deployment
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse::ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response.0.status, "ok");
    }
}
