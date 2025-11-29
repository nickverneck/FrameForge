//! FrameForge Server - Main Entry Point
//!
//! This is the main entry point for the FrameForge Axum-based server.
//! It initializes logging, loads configuration, sets up the router with middleware,
//! and starts the HTTP server.

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import modules from the library
use frameforge_server::config::AppConfig;

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

    // Set up CORS middleware
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

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
    };

    // Build the Axum router
    // For now, we only have placeholder routes - actual endpoints will be added in Phase 3
    let app = Router::new()
        // Placeholder health check endpoint
        .route("/api/health", get(health_handler))
        .route("/", get(root_handler))
        // Add tracing middleware for request/response logging
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
        // Add CORS middleware
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

/// Placeholder health check handler
///
/// This is a simple handler that returns a 200 OK response.
/// The full implementation will be in routes/health.rs (Phase 3).
async fn health_handler() -> &'static str {
    "OK"
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
