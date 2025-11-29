//! Service layer for AI provider integrations.
//!
//! This module implements the service layer pattern for image editing operations.
//! It provides a unified interface (ImageEditor trait) for multiple AI providers:
//! - Google Gemini (Nano Banana) - Primary provider
//! - Fal.ai - Dynamic model support with fal: prefix
//!
//! The factory pattern is used to instantiate the appropriate service based on
//! provider selection. Services handle API communication, image processing,
//! and error handling for their respective providers.

// Base trait and factory (Tasks 11-12)
pub mod base;
pub mod factory;

// Provider implementations
pub mod google_nano_banana; // Tasks 13-14, 21
pub mod fal_editor; // Tasks 15-20, 22
