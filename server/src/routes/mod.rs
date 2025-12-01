//! API route handlers for the FrameForge server.
//!
//! This module contains all HTTP endpoint handlers for the Axum web server.
//! Routes are organized by functionality:
//! - Health check endpoints for monitoring
//! - Provider listing endpoints to show available AI services
//! - Image editing endpoints for AI-powered image manipulation
//!
//! Each route module implements request handling, validation, and response formatting.

/// Health check endpoint
pub mod health;

/// Providers listing endpoint
pub mod providers;

/// Image editing endpoint
pub mod edit;
