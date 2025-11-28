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

// Future service modules will be declared here:
// pub mod base;
// pub mod factory;
// pub mod google_nano_banana;
// pub mod fal_editor;
