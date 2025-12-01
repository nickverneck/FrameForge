//! FrameForge Server - Main Entry Point
//!
//! This is the main entry point for the FrameForge Axum-based server.
//! It initializes logging, loads configuration, sets up the router with middleware,
//! and starts the HTTP server.

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import modules from the library
use frameforge_server::config::AppConfig;
use frameforge_server::middleware::RateLimiter;
use frameforge_server::routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Task 8: Initialize tracing/logging
    // Set up tracing with environment filter support
    // This allows control via RUST_LOG environment variable (e.g., RUST_LOG=debug)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    // Default to INFO level logging
                    "info,frameforge_server=debug,tower_http=debug".into()
                }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    tracing::info!("Starting FrameForge server...");

    // Load configuration from environment variables
    let config = AppConfig::load()?;

    // Log configuration (without sensitive data)
    tracing::info!(
        host = %config.host,
        port = config.port,
        model_id = %config.google_model_id,
        allowed_origins = ?config.allowed_origins,
        "Configuration loaded"
    );

    // Task 34: Set up CORS middleware to match Python backend
    // Python backend uses: allow_credentials=True, allow_methods=["*"], allow_headers=["*"]
    // Note: When allow_credentials is true, we must specify headers explicitly (not Any)
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use axum::http::Method;

    let cors = if config.allowed_origins.contains(&"*".to_string()) {
        tracing::warn!("CORS configured with wildcard (*) - allowing all origins");
        CorsLayer::permissive()
    } else {
        tracing::info!("CORS configured with specific origins: {:?}", config.allowed_origins);
        let origins = config
            .allowed_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect::<Vec<_>>();

        // When using allow_credentials, we must specify headers explicitly
        let allowed_headers = vec![
            AUTHORIZATION,
            CONTENT_TYPE,
            "x-google-api-key".parse().unwrap(),
            "x-gemini-api-key".parse().unwrap(),
            "x-fal-key".parse().unwrap(),
        ];

        CorsLayer::new()
            .allow_origin(origins)
            .allow_credentials(true)
            .allow_methods(vec![
                Method::GET,
                Method::POST,
                Method::OPTIONS,
            ])
            .allow_headers(allowed_headers)
    };

    // Task 41: Create rate limiter (implementation available in middleware::rate_limit)
    // Note: Rate limiting middleware is implemented but not yet integrated into the router
    // It can be added later by using axum::middleware::from_fn with rate_limit_middleware
    let _rate_limiter = RateLimiter::new();

    // Build the Axum router with all API endpoints
    // Middleware layers are applied in reverse order (bottom executes first)
    let app = Router::new()
        // API routes (Task 33)
        .route("/api/health", get(routes::health::health_check))
        .route("/api/providers", get(routes::providers::list_providers))
        .route("/api/edit", post(routes::edit::edit_image))
        // Root endpoint
        .route("/", get(root_handler))
        // Add AppConfig to shared state for dependency injection
        .with_state(config.clone())
        // Task 37: Add request size limits (50MB for image uploads)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB
        // Task 40: Add timeout layers (different timeouts for different endpoints)
        // Edit endpoint gets 5 minutes for AI processing
        // Returns 408 Request Timeout on timeout
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(300) // 5 minutes for AI processing
                ))
        )
        // Task 35: Add enhanced tracing middleware for request/response logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                        .level(Level::INFO),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(LatencyUnit::Millis)
                        .level(Level::INFO),
                ),
        )
        // Task 36: Add compression middleware (br/brotli and gzip)
        .layer(CompressionLayer::new().br(true).gzip(true))
        // Task 34: Add CORS middleware
        .layer(cors);

    // Bind to the configured host and port
    let addr = SocketAddr::new(
        config.host.parse()?,
        config.port,
    );

    tracing::info!("Server listening on {}", addr);

    // Start the server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

/// Root handler for the server
///
/// Returns basic information about the server.
async fn root_handler() -> &'static str {
    "FrameForge Server - Axum Implementation"
}

/// Graceful shutdown signal handler
///
/// This function listens for SIGTERM and SIGINT signals (Ctrl+C)
/// and triggers graceful shutdown when received.
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, starting graceful shutdown");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal, starting graceful shutdown");
        },
    }
}
