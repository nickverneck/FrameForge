//! Data models and DTOs (Data Transfer Objects) for the API.
//!
//! This module contains all request and response structures used throughout
//! the application. Models are defined with serde serialization/deserialization
//! support for JSON handling.
//!
//! Request models define incoming API payloads including multipart form data.
//! Response models define outgoing JSON structures for consistent API responses.

/// Request payload models
pub mod request;

/// Response payload models
pub mod response;
