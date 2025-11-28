//! FrameForge Server Library
//!
//! This library provides the core functionality for the FrameForge image editing server.
//! Built with Axum and Tokio, it offers high-performance AI-powered image editing
//! through multiple provider integrations.
//!
//! # Architecture
//!
//! The application is structured into several key modules:
//! - `routes`: HTTP endpoint handlers
//! - `services`: AI provider service implementations
//! - `models`: Request/response data structures
//! - `utils`: Shared utility functions
//!
//! # Features
//!
//! - Multi-provider AI image editing (Google Gemini, Fal.ai)
//! - Multipart file upload support
//! - Streaming image responses
//! - Flexible configuration management
//! - Comprehensive error handling

// Module declarations - these modules will be implemented in subsequent tasks

/// Common error types and helpers
pub mod error;

/// API route handlers for HTTP endpoints
pub mod routes;

/// Service layer for AI provider integrations
pub mod services;

/// Data models and DTOs for request/response handling
pub mod models;

/// Utility functions and helpers
pub mod utils;
