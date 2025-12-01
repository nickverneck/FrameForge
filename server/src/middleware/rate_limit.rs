//! Rate limiting middleware
//!
//! This module implements IP-based rate limiting to prevent API abuse.
//! Different endpoints have different rate limits:
//! - /api/edit: 100 requests/hour per IP
//! - Other endpoints: 1000 requests/hour per IP
//!
//! Security: Never logs IP addresses alongside API keys

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limit configuration
const EDIT_LIMIT: usize = 100; // requests per hour for /api/edit
const GENERAL_LIMIT: usize = 1000; // requests per hour for other endpoints
const WINDOW_DURATION: Duration = Duration::from_secs(3600); // 1 hour

/// Rate limit entry for an IP address
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: usize,
    window_start: Instant,
}

/// Rate limiter state
#[derive(Debug, Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a request should be allowed
    async fn check_rate_limit(&self, ip: &str, path: &str) -> Result<(), Duration> {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        // Determine limit based on endpoint
        let limit = if path.starts_with("/api/edit") {
            EDIT_LIMIT
        } else {
            GENERAL_LIMIT
        };

        // Get or create entry for this IP
        let entry = state.entry(ip.to_string()).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) > WINDOW_DURATION {
            entry.count = 0;
            entry.window_start = now;
        }

        // Check limit
        if entry.count >= limit {
            let retry_after = WINDOW_DURATION
                .checked_sub(now.duration_since(entry.window_start))
                .unwrap_or(Duration::from_secs(0));
            return Err(retry_after);
        }

        // Increment count
        entry.count += 1;

        Ok(())
    }

    /// Clean up expired entries (optional optimization)
    #[allow(dead_code)]
    async fn cleanup(&self) {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        state.retain(|_, entry| now.duration_since(entry.window_start) <= WINDOW_DURATION);
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, impl IntoResponse> {
    let ip = addr.ip().to_string();
    let path = request.uri().path().to_string();

    // Get rate limiter from request extensions (must be added in main.rs)
    let limiter = request
        .extensions()
        .get::<RateLimiter>()
        .cloned()
        .unwrap_or_else(RateLimiter::new);

    match limiter.check_rate_limit(&ip, &path).await {
        Ok(()) => {
            // Request allowed
            Ok(next.run(request).await)
        }
        Err(retry_after) => {
            // Rate limit exceeded
            let retry_after_secs = retry_after.as_secs();

            // Log rate limit hit (without any sensitive data like API keys)
            tracing::warn!(
                "Rate limit exceeded for IP: {} on path: {}",
                ip,
                path
            );

            // Return 429 Too Many Requests with Retry-After header
            let response = Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header("Retry-After", retry_after_secs.to_string())
                .header("Content-Type", "application/json")
                .body(Body::from(format!(
                    r#"{{"error":"Rate limit exceeded","retry_after_seconds":{}}}"#,
                    retry_after_secs
                )))
                .unwrap();

            Err(response)
        }
    }
}
