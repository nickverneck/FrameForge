//! Middleware modules
//!
//! This module contains custom middleware for the FrameForge server.

pub mod rate_limit;

pub use rate_limit::{rate_limit_middleware, RateLimiter};
