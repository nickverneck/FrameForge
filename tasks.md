# FrameForge Rust Server Implementation Tasks

This document breaks down the implementation of the Rust server into small, manageable tasks that can be accomplished by individual agents. Tasks are organized by phase and module, with clear dependencies and acceptance criteria.

**Total Tasks**: 52
**Parallel Execution Opportunities**: Multiple tasks within each phase can run in parallel
**Framework**: Axum + Tokio (replacing the existing Rocket implementation)

---

## PHASE 1: Foundation & Project Setup (10 tasks)

### Task 1: Update Cargo.toml with Complete Dependencies
**Phase**: 1
**Module**: Project Configuration
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/Cargo.toml

**Description**:
Replace the existing Rocket-based Cargo.toml with the complete dependency list for Axum-based implementation. This includes web framework (axum), async runtime (tokio), serialization (serde), HTTP client (reqwest), image processing (image), configuration (config, dotenvy), error handling (anyhow, thiserror), logging (tracing), AI providers (rust-genai), and multipart handling (axum-typed-multipart).

**Acceptance Criteria**:
- [ ] All dependencies from architecture.md are included with correct versions
- [ ] Package name is "frameforge-server"
- [ ] Edition is set to "2021" (not 2024 as in current file)
- [ ] Dev dependencies section includes tower utilities for testing
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: None (first task)

---

### Task 2: Create .env.example Configuration Template
**Phase**: 1
**Module**: Configuration
**Can run in parallel with**: Task 3, Task 4
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/.env.example

**Description**:
Create an example environment configuration file that documents all required and optional environment variables. This should include API keys for Google Gemini and Fal.ai, model configuration, server settings, and CORS configuration. This file serves as a template for developers.

**Acceptance Criteria**:
- [ ] Contains all config fields from app/config.py analysis
- [ ] Includes GOOGLE_API_KEY, GEMINI_API_KEY, FAL_KEY (optional)
- [ ] Includes GOOGLE_MODEL_ID with default value
- [ ] Includes ALLOWED_ORIGINS with example values
- [ ] Includes SERVER_HOST and SERVER_PORT
- [ ] Has helpful comments explaining each variable
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 3: Create Project Directory Structure
**Phase**: 1
**Module**: Project Organization
**Can run in parallel with**: Task 2, Task 4
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/lib.rs
- /Users/nick/Documents/dev/FrameForge/server/src/config.rs
- /Users/nick/Documents/dev/FrameForge/server/src/error.rs
- /Users/nick/Documents/dev/FrameForge/server/src/routes/mod.rs
- /Users/nick/Documents/dev/FrameForge/server/src/services/mod.rs
- /Users/nick/Documents/dev/FrameForge/server/src/models/mod.rs
- /Users/nick/Documents/dev/FrameForge/server/src/utils/mod.rs

**Description**:
Create all module files with empty placeholder content and proper module declarations. Each mod.rs should export submodules, and lib.rs should declare all top-level modules. This establishes the project structure before implementation.

**Acceptance Criteria**:
- [ ] All directories created as per architecture.md structure
- [ ] Each module has a mod.rs file with appropriate declarations
- [ ] lib.rs declares all top-level modules (config, error, routes, services, models, utils)
- [ ] Empty implementations compile without errors
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 4: Implement Configuration Management (config.rs)
**Phase**: 1
**Module**: Configuration
**Can run in parallel with**: Task 2, Task 3
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/config.rs

**Description**:
Implement the AppConfig struct using serde for deserialization from environment variables. Use dotenvy to load .env file, and config crate for flexible configuration sources. Implement validation logic and provide default values matching the Python backend's behavior. Follow specs/app_config.md.

**Acceptance Criteria**:
- [ ] AppConfig struct defined with all fields from .env.example
- [ ] Uses serde Deserialize derive macro
- [ ] Implements load() method using dotenvy and config crate
- [ ] Provides sensible defaults for optional fields
- [ ] Validates required fields at startup
- [ ] API keys are Option<String> types
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 5: Implement Custom Error Types (error.rs)
**Phase**: 1
**Module**: Error Handling
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/error.rs

**Description**:
Define custom error types using thiserror for domain errors. Implement IntoResponse trait for Axum error handling. Create error variants for invalid image format, provider not found, AI service errors, configuration errors, and internal server errors. Include appropriate HTTP status codes and JSON error responses.

**Acceptance Criteria**:
- [ ] ApiError enum defined with thiserror::Error derive
- [ ] Error variants: InvalidImageFormat, ProviderNotFound, AiServiceError, ConfigError, Internal
- [ ] IntoResponse trait implemented for ApiError
- [ ] Returns appropriate HTTP status codes (400, 404, 500, etc.)
- [ ] Error responses are JSON formatted
- [ ] Includes error context for debugging
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 6: Create Request DTOs (models/request.rs)
**Phase**: 1
**Module**: Data Models
**Can run in parallel with**: Task 7
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/models/request.rs
- /Users/nick/Documents/dev/FrameForge/server/src/models/mod.rs

**Description**:
Define request data transfer objects using serde. Create EditRequest struct for multipart form data including image files, text prompt, optional provider selection, and any additional options. Use axum-typed-multipart for type-safe multipart handling.

**Acceptance Criteria**:
- [ ] EditRequest struct defined with serde Deserialize
- [ ] Fields for images (Vec of uploaded files), prompt (String), provider (Option<String>)
- [ ] Uses axum_typed_multipart::TypedMultipart or similar
- [ ] Includes validation attributes (size limits, format checks)
- [ ] models/mod.rs exports request module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 7: Create Response DTOs (models/response.rs)
**Phase**: 1
**Module**: Data Models
**Can run in parallel with**: Task 6
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/models/response.rs
- /Users/nick/Documents/dev/FrameForge/server/src/models/mod.rs

**Description**:
Define response data transfer objects using serde. Create HealthResponse, ProvidersResponse, and any other structured JSON responses. These should match the Python backend's response formats exactly for API compatibility.

**Acceptance Criteria**:
- [ ] HealthResponse struct with status field (returns "ok")
- [ ] ProvidersResponse as Vec<String> or similar
- [ ] All responses implement serde Serialize
- [ ] Matches Python FastAPI response formats
- [ ] models/mod.rs exports response module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 8: Implement Logging and Tracing Setup
**Phase**: 1
**Module**: Infrastructure
**Can run in parallel with**: Task 9, Task 10
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (partial)

**Description**:
Set up tracing-subscriber with environment filter and JSON formatting capabilities. Configure log levels (INFO for production, DEBUG for development). This should be initialized at the start of main() before any other operations.

**Acceptance Criteria**:
- [ ] tracing_subscriber initialized in main()
- [ ] Environment filter configured (reads RUST_LOG env var)
- [ ] JSON formatting support enabled
- [ ] Includes timestamp and target information
- [ ] Works with tower-http tracing middleware
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 9: Create Image Utility Functions (utils/image_utils.rs)
**Phase**: 1
**Module**: Utilities
**Can run in parallel with**: Task 8, Task 10
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/utils/image_utils.rs
- /Users/nick/Documents/dev/FrameForge/server/src/utils/mod.rs

**Description**:
Implement utility functions for image processing using the image crate. Include functions for: validating image format, loading images from bytes, converting between formats, generating base64 data URLs, and determining MIME types. These will be used by service implementations.

**Acceptance Criteria**:
- [ ] validate_image_bytes() function checks if bytes are valid image
- [ ] bytes_to_image() loads image from bytes
- [ ] image_to_bytes() saves image to bytes
- [ ] bytes_to_base64() creates data URL for API calls
- [ ] get_mime_type() determines MIME type from image data
- [ ] All functions handle errors gracefully
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1 must be completed first

---

### Task 10: Implement Basic Main.rs Structure
**Phase**: 1
**Module**: Application Entry Point
**Can run in parallel with**: Task 8, Task 9
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs

**Description**:
Replace the existing Rocket-based main.rs with basic Axum server structure. Set up tokio runtime, load configuration, initialize tracing, create a minimal Axum router (no routes yet), and bind the server. This creates a running server foundation that future tasks will build upon.

**Acceptance Criteria**:
- [ ] Replaces Rocket imports with Axum
- [ ] #[tokio::main] async fn main() -> Result<()>
- [ ] Loads AppConfig using config module
- [ ] Initializes tracing/logging
- [ ] Creates basic Axum Router::new()
- [ ] Binds to host:port from config
- [ ] Server starts successfully
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 1, Task 4, Task 5, Task 8 must be completed first

---

## PHASE 2: Service Layer Implementation (13 tasks)

### Task 11: Define ImageEditor Trait (services/base.rs)
**Phase**: 2
**Module**: Service Layer - Base
**Can run in parallel with**: Task 12
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/base.rs
- /Users/nick/Documents/dev/FrameForge/server/src/services/mod.rs

**Description**:
Define the ImageEditor trait that all AI provider implementations will implement. Use async_trait for async methods. Define edit_image method signature accepting Vec<bytes::Bytes> for images, String for prompt, and HashMap<String, String> for options. Return Result with edited image bytes and optional MIME type. Follow specs/app_services_base.md.

**Acceptance Criteria**:
- [ ] ImageEditor trait defined with async_trait
- [ ] edit_image async method signature defined
- [ ] Trait requires Send + Sync bounds for concurrent usage
- [ ] Uses ApiError for error type
- [ ] Returns (bytes::Bytes, Option<String>) for image and MIME type
- [ ] services/mod.rs exports base module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 1 (Tasks 1-10) must be completed first

---

### Task 12: Create Service Factory Structure (services/factory.rs)
**Phase**: 2
**Module**: Service Layer - Factory
**Can run in parallel with**: Task 11
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/factory.rs
- /Users/nick/Documents/dev/FrameForge/server/src/services/mod.rs

**Description**:
Implement the factory pattern for creating ImageEditor instances. Create list_providers() function returning available providers, and get_editor() function that takes provider name and returns appropriate Box<dyn ImageEditor>. Handle dynamic fal: prefix parsing. Provide default provider logic. Follow specs/app_services_factory.md.

**Acceptance Criteria**:
- [ ] list_providers() returns Vec<String> of static providers
- [ ] get_editor() function signature defined
- [ ] Accepts provider_name, config, and runtime_api_keys
- [ ] Returns Box<dyn ImageEditor>
- [ ] Parses "fal:" prefix for dynamic models
- [ ] Has PROVIDERS static HashMap for name -> constructor mapping
- [ ] Defaults to Google provider if not found
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 1 (Tasks 1-10) must be completed first

---

### Task 13: Implement Google Gemini Client Initialization
**Phase**: 2
**Module**: Service Layer - Google Provider
**Can run in parallel with**: Task 14
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/google_nano_banana.rs (partial)
- /Users/nick/Documents/dev/FrameForge/server/src/services/mod.rs

**Description**:
Create the GoogleNanaBananaEditor struct and implement initialization logic. Set up rust-genai client for Google Gemini API. Handle API key from config or runtime parameters. Create constructor that validates API key presence and initializes the client. Follow specs/app_services_google_nano_banana.md.

**Acceptance Criteria**:
- [ ] GoogleNanaBananaEditor struct defined
- [ ] Holds rust-genai client or similar
- [ ] new() constructor accepts AppConfig and optional runtime keys
- [ ] Validates API key is present (from config or runtime)
- [ ] Initializes Gemini client with proper configuration
- [ ] Uses GOOGLE_MODEL_ID from config
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 11 must be completed first

---

### Task 14: Implement Google Gemini Image Editing Logic
**Phase**: 2
**Module**: Service Layer - Google Provider
**Can run in parallel with**: Task 13
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/google_nano_banana.rs

**Description**:
Implement the edit_image method for GoogleNanaBananaEditor. Convert images to format required by Gemini API (likely base64), make API call with prompt, handle streaming response if applicable, and return edited image bytes. Use tokio::spawn_blocking if Gemini SDK is synchronous. Include fallback behavior (return original image) if no API key in development mode.

**Acceptance Criteria**:
- [ ] Implements ImageEditor trait for GoogleNanaBananaEditor
- [ ] edit_image method converts images to base64 using utils
- [ ] Makes Gemini API call with prompt and images
- [ ] Handles API response and extracts image data
- [ ] Converts response back to bytes
- [ ] Returns appropriate MIME type
- [ ] Includes development mode fallback
- [ ] Proper error handling with ApiError::AiServiceError
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 11, Task 13 must be completed first

---

### Task 15: Implement Fal.ai Client Initialization
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: Task 16
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs (partial)
- /Users/nick/Documents/dev/FrameForge/server/src/services/mod.rs

**Description**:
Create the FalImageEditor struct and implement initialization logic. Set up reqwest HTTP client for Fal.ai API calls. Store model_path extracted from provider string (e.g., "fal:fal-ai/flux/dev" -> "fal-ai/flux/dev"). Handle API key from config or runtime parameters. Follow specs/app_services_fal_editor.md.

**Acceptance Criteria**:
- [ ] FalImageEditor struct defined
- [ ] Holds reqwest::Client and model_path
- [ ] new() constructor accepts model_path, AppConfig, and optional runtime keys
- [ ] Validates FAL_KEY is present
- [ ] Initializes reqwest client
- [ ] Stores model_path for API calls
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 11 must be completed first

---

### Task 16: Implement Fal.ai Image Upload Logic
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: Task 15
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs (partial)

**Description**:
Implement helper method for uploading images to Fal.ai storage. Use reqwest multipart to upload image bytes, handle upload response, and return image URLs that can be used in generation requests. This is a prerequisite for the main edit_image implementation.

**Acceptance Criteria**:
- [ ] upload_image() helper method defined
- [ ] Takes image bytes and uploads to Fal.ai storage endpoint
- [ ] Uses reqwest multipart for upload
- [ ] Includes FAL_KEY in authorization header
- [ ] Handles upload response and extracts URL
- [ ] Returns image URL as String
- [ ] Proper error handling with ApiError
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 11, Task 15 must be completed first

---

### Task 17: Implement Fal.ai Generation Request Logic
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs (partial)

**Description**:
Implement helper method for submitting generation requests to Fal.ai. Construct request body with uploaded image URLs and prompt, submit to model endpoint, handle queued/processing response, and return job ID or status URL for polling.

**Acceptance Criteria**:
- [ ] submit_generation() helper method defined
- [ ] Constructs JSON request body with image URLs and prompt
- [ ] Posts to Fal.ai model endpoint (using model_path)
- [ ] Includes FAL_KEY in authorization header
- [ ] Handles response and extracts job ID or status URL
- [ ] Returns job identifier for polling
- [ ] Proper error handling with ApiError
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 16 must be completed first

---

### Task 18: Implement Fal.ai Job Polling Logic
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs (partial)

**Description**:
Implement helper method for polling Fal.ai job status. Implement exponential backoff, check for completion/failure states, handle timeout scenarios, and extract result image URL when complete.

**Acceptance Criteria**:
- [ ] poll_job() helper method defined
- [ ] Takes job ID and polls status endpoint
- [ ] Implements exponential backoff (100ms, 200ms, 400ms, etc.)
- [ ] Checks for completed, failed, and processing states
- [ ] Times out after reasonable duration (60 seconds)
- [ ] Returns result URL when completed
- [ ] Proper error handling for failed jobs
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 17 must be completed first

---

### Task 19: Implement Fal.ai Image Download Logic
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs (partial)

**Description**:
Implement helper method for downloading result images from Fal.ai URLs. Use reqwest to fetch image, stream response into bytes::Bytes, and handle download errors. This completes the Fal.ai implementation chain.

**Acceptance Criteria**:
- [ ] download_image() helper method defined
- [ ] Takes image URL and fetches content
- [ ] Uses reqwest streaming to handle large images efficiently
- [ ] Returns bytes::Bytes
- [ ] Validates response is successful
- [ ] Determines MIME type from response headers
- [ ] Proper error handling for download failures
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 18 must be completed first

---

### Task 20: Implement Fal.ai Edit Image Method
**Phase**: 2
**Module**: Service Layer - Fal Provider
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/fal_editor.rs

**Description**:
Implement the main edit_image method for FalImageEditor. Orchestrate the complete workflow: upload images, submit generation request, poll for completion, download result, and return bytes. Implement ImageEditor trait.

**Acceptance Criteria**:
- [ ] Implements ImageEditor trait for FalImageEditor
- [ ] edit_image orchestrates full workflow
- [ ] Calls upload_image for each input image
- [ ] Calls submit_generation with uploaded URLs
- [ ] Calls poll_job to wait for completion
- [ ] Calls download_image to fetch result
- [ ] Returns (bytes::Bytes, Option<String>) with MIME type
- [ ] Proper error handling throughout
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 19 must be completed first

---

### Task 21: Wire Google Provider into Factory
**Phase**: 2
**Module**: Service Layer - Factory
**Can run in parallel with**: Task 22
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/factory.rs

**Description**:
Update the factory to instantiate GoogleNanaBananaEditor. Add entries to PROVIDERS map for "google" and "nano-banana" that create GoogleNanaBananaEditor instances with appropriate configuration. Set as default provider.

**Acceptance Criteria**:
- [ ] PROVIDERS map includes "google" key
- [ ] PROVIDERS map includes "nano-banana" key
- [ ] Both map to GoogleNanaBananaEditor constructor
- [ ] Pass AppConfig and runtime keys correctly
- [ ] Default case returns GoogleNanaBananaEditor
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 12, Task 13, Task 14 must be completed first

---

### Task 22: Wire Fal Provider into Factory
**Phase**: 2
**Module**: Service Layer - Factory
**Can run in parallel with**: Task 21
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/factory.rs

**Description**:
Update the factory to handle fal: prefixed providers. Parse model path from provider string (e.g., "fal:fal-ai/flux/dev"), instantiate FalImageEditor with extracted model path, and return as Box<dyn ImageEditor>.

**Acceptance Criteria**:
- [ ] get_editor checks for "fal:" prefix
- [ ] Parses model path from provider string
- [ ] Creates FalImageEditor with extracted path
- [ ] Passes AppConfig and runtime keys correctly
- [ ] Handles invalid fal: format gracefully
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 12, Task 20 must be completed first

---

### Task 23: Complete Factory list_providers Implementation
**Phase**: 2
**Module**: Service Layer - Factory
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/services/factory.rs

**Description**:
Complete the list_providers function to return all statically available providers. Include documentation about dynamic fal: providers not being listed. This should match the Python backend's provider listing behavior.

**Acceptance Criteria**:
- [ ] Returns Vec<String> of all provider names
- [ ] Includes "google", "nano-banana"
- [ ] Includes example dynamic fal: providers (e.g., "fal:fal-ai/flux/dev")
- [ ] Returns sorted list for consistency
- [ ] Matches Python backend provider list exactly
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 12, Task 21, Task 22 must be completed first

---

## PHASE 3: API Routes Implementation (10 tasks)

### Task 24: Implement Health Check Endpoint (routes/health.rs)
**Phase**: 3
**Module**: Routes - Health
**Can run in parallel with**: Task 25
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/health.rs
- /Users/nick/Documents/dev/FrameForge/server/src/routes/mod.rs

**Description**:
Implement GET /api/health endpoint using Axum. Return JSON response with {"status": "ok"}. This should exactly match the Python backend response format. Use the HealthResponse model from models/response.rs.

**Acceptance Criteria**:
- [ ] health_check handler function defined
- [ ] Returns Json<HealthResponse>
- [ ] Response contains status: "ok"
- [ ] No parameters required
- [ ] Matches Python FastAPI response format
- [ ] routes/mod.rs exports health module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 2 (Tasks 11-23) must be completed first

---

### Task 25: Implement Providers Listing Endpoint (routes/providers.rs)
**Phase**: 3
**Module**: Routes - Providers
**Can run in parallel with**: Task 24
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/providers.rs
- /Users/nick/Documents/dev/FrameForge/server/src/routes/mod.rs

**Description**:
Implement GET /api/providers endpoint using Axum. Call factory's list_providers() function and return as JSON array. Include both static providers and examples of dynamic fal: providers. Match Python backend response format.

**Acceptance Criteria**:
- [ ] providers_list handler function defined
- [ ] Calls services::factory::list_providers()
- [ ] Returns Json<Vec<String>>
- [ ] Includes static providers (google, nano-banana)
- [ ] Includes dynamic fal: examples (e.g., "fal:fal-ai/flux/dev")
- [ ] Matches Python FastAPI response format exactly
- [ ] routes/mod.rs exports providers module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 2 (Tasks 11-23) must be completed first

---

### Task 26: Implement Multipart Form Extraction Logic
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (partial)
- /Users/nick/Documents/dev/FrameForge/server/src/routes/mod.rs

**Description**:
Implement multipart form data extraction for the edit endpoint. Use axum_typed_multipart or axum::extract::Multipart to parse uploaded images, prompt text, provider selection, and any additional options. Validate that uploaded files are images.

**Acceptance Criteria**:
- [ ] EditRequest struct defined with TypedMultipart or similar
- [ ] Extracts multiple image file uploads
- [ ] Extracts prompt field (String)
- [ ] Extracts optional provider field
- [ ] Validates files are images using utils::image_utils
- [ ] Handles missing fields with defaults
- [ ] Returns proper errors for invalid data
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 2 (Tasks 11-23) must be completed first

---

### Task 27: Implement API Key Header Extraction
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: Task 26
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (partial)

**Description**:
Implement logic to extract API keys from request headers. Support X-Google-API-Key, X-Gemini-API-Key, and X-Fal-Key headers. Store in HashMap for passing to service factory. This allows per-request API key override.

**Acceptance Criteria**:
- [ ] Extracts optional API keys from headers
- [ ] Supports X-Google-API-Key header
- [ ] Supports X-Gemini-API-Key header
- [ ] Supports X-Fal-Key header
- [ ] Stores in HashMap<String, String>
- [ ] Falls back to config if headers not present
- [ ] Never logs API keys
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 2 (Tasks 11-23) must be completed first

---

### Task 28: Implement Provider Selection Logic
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (partial)

**Description**:
Implement logic to select the appropriate AI provider based on request data. Extract provider from form data or query parameter, default to "google" if not specified, and call factory::get_editor() with provider name and configuration.

**Acceptance Criteria**:
- [ ] Extracts provider from request (form field or query param)
- [ ] Defaults to "google" if not specified
- [ ] Calls factory::get_editor() with provider name
- [ ] Passes AppConfig and runtime_api_keys to factory
- [ ] Handles ProviderNotFound error
- [ ] Returns appropriate error response
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 26, Task 27 must be completed first

---

### Task 29: Implement Default Prompt Logic
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: Task 28
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (partial)

**Description**:
Implement default prompt handling. If no prompt is provided in the request, use the default prompt from Python backend: "Please furnish this room with minimalist modern furniture that complements the space." This ensures API compatibility.

**Acceptance Criteria**:
- [ ] Checks if prompt field is empty or missing
- [ ] Uses default prompt if not provided
- [ ] Default matches Python backend exactly
- [ ] Allows user prompt to override default
- [ ] Trims whitespace from prompt
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 26 must be completed first

---

### Task 30: Implement Image Editing Service Call
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (partial)

**Description**:
Implement the core logic to call the selected ImageEditor service. Convert uploaded images to Vec<bytes::Bytes>, call editor.edit_image() with images and prompt, handle async execution, and catch errors appropriately.

**Acceptance Criteria**:
- [ ] Converts uploaded files to Vec<bytes::Bytes>
- [ ] Calls editor.edit_image() with images, prompt, and options
- [ ] Awaits async result
- [ ] Handles ApiError from service layer
- [ ] Logs errors with tracing
- [ ] Includes request context in error logs
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 28, Task 29 must be completed first

---

### Task 31: Implement Streaming Image Response
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs

**Description**:
Implement streaming image response using Axum body streaming. Set appropriate Content-Type header based on MIME type from service, stream bytes efficiently without loading entire image in memory, and handle streaming errors.

**Acceptance Criteria**:
- [ ] Returns Response with streaming body
- [ ] Sets Content-Type header from MIME type
- [ ] Uses Body::from(bytes) or StreamBody for efficient streaming
- [ ] Handles large images without memory issues
- [ ] Includes Content-Length header if known
- [ ] Matches Python backend streaming behavior
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 30 must be completed first

---

### Task 32: Complete Edit Endpoint Implementation
**Phase**: 3
**Module**: Routes - Edit Endpoint
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs

**Description**:
Complete the POST /api/edit endpoint by integrating all previous task components. Implement full request-to-response flow: extract form data, validate images, select provider, call service, stream response. Add comprehensive error handling and logging.

**Acceptance Criteria**:
- [ ] edit_image handler function complete
- [ ] Integrates multipart extraction, provider selection, service call
- [ ] Full error handling for all error cases
- [ ] Tracing/logging at appropriate points
- [ ] Matches Python FastAPI behavior exactly
- [ ] routes/mod.rs exports edit module
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 31 must be completed first

---

### Task 33: Wire Routes into Main Router
**Phase**: 3
**Module**: Application Entry Point
**Can run in parallel with**: none
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs

**Description**:
Update main.rs to register all route handlers with Axum router. Mount /api/health, /api/providers, and /api/edit endpoints. Ensure path prefixes match Python backend. Include AppConfig in application state for dependency injection.

**Acceptance Criteria**:
- [ ] Router includes /api/health route
- [ ] Router includes /api/providers route
- [ ] Router includes /api/edit route
- [ ] AppConfig added to Axum state via with_state()
- [ ] Path structure matches Python backend
- [ ] All routes accessible
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Task 24, Task 25, Task 32 must be completed first

---

## PHASE 4: Middleware & Production Features (9 tasks)

**Note**: Tasks 34-38 all modify src/main.rs and should be coordinated to avoid merge conflicts. Consider implementing them sequentially or with careful coordination.

### Task 34: Implement CORS Middleware
**Phase**: 4
**Module**: Middleware
**Can run in parallel with**: none (coordinates with Tasks 35-38)
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (middleware section)

**Description**:
Add tower-http CORS layer to Axum router. Load allowed origins from AppConfig, support wildcard in development, restrict to specific origins in production. Match Python backend CORS configuration exactly.

**Acceptance Criteria**:
- [ ] Uses tower_http::cors::CorsLayer
- [ ] Loads allowed_origins from config
- [ ] Supports wildcard ["*"] for development
- [ ] Allows credentials if needed
- [ ] Allows required headers (Content-Type, X-*-API-Key)
- [ ] Allows required methods (GET, POST, OPTIONS)
- [ ] Matches Python FastAPI CORS settings
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 35: Implement Tracing Middleware
**Phase**: 4
**Module**: Middleware
**Can run in parallel with**: none (coordinates with Tasks 34, 36-38)
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (middleware section)

**Description**:
Add tower-http tracing layer to Axum router. Log all requests with method, path, status code, and duration. Include request ID for correlation. Configure appropriate log levels for different types of requests.

**Acceptance Criteria**:
- [ ] Uses tower_http::trace::TraceLayer
- [ ] Logs request method and path
- [ ] Logs response status and duration
- [ ] Includes request ID in logs
- [ ] Configured with appropriate log levels
- [ ] Integrates with tracing setup from Task 8
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 36: Implement Compression Middleware
**Phase**: 4
**Module**: Middleware
**Can run in parallel with**: none (coordinates with Tasks 34-35, 37-38)
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (middleware section)

**Description**:
Add tower-http compression layer to Axum router. Support gzip and br (brotli) compression for responses. Configure appropriate compression levels. Don't compress images unnecessarily (they're already compressed).

**Acceptance Criteria**:
- [ ] Uses tower_http::compression::CompressionLayer
- [ ] Supports gzip compression
- [ ] Supports brotli compression if available
- [ ] Configures reasonable compression level
- [ ] Doesn't double-compress images
- [ ] Works with streaming responses
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 37: Implement Request Size Limits
**Phase**: 4
**Module**: Middleware
**Can run in parallel with**: none (coordinates with Tasks 34-36, 38)
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (middleware section)
- /Users/nick/Documents/dev/FrameForge/server/src/routes/edit.rs (size limit config)

**Description**:
Configure request body size limits to prevent abuse. Set reasonable limits for multipart uploads (e.g., 50MB total, 10MB per file). Use Axum's DefaultBodyLimit or custom middleware. Return appropriate error when exceeded.

**Acceptance Criteria**:
- [ ] Sets global body size limit
- [ ] Configures per-route limits if needed
- [ ] Limits are reasonable for image uploads (50MB total)
- [ ] Returns 413 Payload Too Large on limit exceeded
- [ ] Error message is helpful
- [ ] Documented in API docs
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 38: Implement Graceful Shutdown
**Phase**: 4
**Module**: Application Entry Point
**Can run in parallel with**: none (coordinates with Tasks 34-37)
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs

**Description**:
Implement graceful shutdown handling. Listen for SIGTERM/SIGINT signals, wait for in-flight requests to complete (with timeout), close connections cleanly, and log shutdown process. This is critical for production deployments.

**Acceptance Criteria**:
- [ ] Listens for SIGTERM and SIGINT
- [ ] Uses tokio::signal for signal handling
- [ ] Waits for in-flight requests with timeout (30s)
- [ ] Closes connections gracefully
- [ ] Logs shutdown process
- [ ] Prevents new requests during shutdown
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 39: Add Server Configuration Validation
**Phase**: 4
**Module**: Configuration
**Can run in parallel with**: Task 38
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/config.rs

**Description**:
Add validation logic to AppConfig that runs at startup. Validate that at least one provider has an API key configured (or development mode is enabled). Warn if configuration seems incomplete. Log all configuration (without API keys) for debugging.

**Acceptance Criteria**:
- [ ] validate() method on AppConfig
- [ ] Checks at least one API key is configured
- [ ] Warns if ALLOWED_ORIGINS is wildcard in production
- [ ] Logs configuration at startup (without keys)
- [ ] Returns error for invalid configuration
- [ ] Called from main() before server starts
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 40: Implement Request Timeout Handling
**Phase**: 4
**Module**: Middleware
**Can run in parallel with**: Task 41
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs (middleware section)

**Description**:
Add request timeout middleware using tower-http TimeoutLayer. Set reasonable timeouts for different endpoints (e.g., 30s for health/providers, 5 minutes for edit endpoint to allow AI processing). Return 408 Request Timeout when exceeded. This prevents hung connections and resource exhaustion.

**Acceptance Criteria**:
- [ ] Uses tower_http::timeout::TimeoutLayer
- [ ] Sets per-route timeouts as appropriate
- [ ] Health/providers endpoints: 30s timeout
- [ ] Edit endpoint: 5 minute timeout for AI processing
- [ ] Returns 408 Request Timeout status code
- [ ] Includes helpful error message
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 41: Implement Rate Limiting Middleware
**Phase**: 4
**Module**: Security & Middleware
**Can run in parallel with**: Task 40, Task 42
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/src/middleware/rate_limit.rs
- /Users/nick/Documents/dev/FrameForge/server/src/middleware/mod.rs
- /Users/nick/Documents/dev/FrameForge/server/src/main.rs

**Description**:
Implement rate limiting middleware to prevent API abuse. Use tower-governor or custom implementation with tokio rate limiting. Configure per-IP limits (e.g., 100 requests/hour for edit endpoint, 1000 requests/hour for other endpoints). Return 429 Too Many Requests when exceeded.

**Acceptance Criteria**:
- [ ] Rate limiting middleware implemented
- [ ] Per-IP tracking using client IP from request
- [ ] Configurable limits per endpoint
- [ ] Edit endpoint: 100 requests/hour per IP
- [ ] Other endpoints: 1000 requests/hour per IP
- [ ] Returns 429 Too Many Requests status code
- [ ] Includes Retry-After header
- [ ] Never logs IP addresses with API keys
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33) must be completed first

---

### Task 42: Add Load Testing and Performance Benchmarks
**Phase**: 4
**Module**: Testing & Performance
**Can run in parallel with**: Task 41
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/benches/api_benchmark.rs
- /Users/nick/Documents/dev/FrameForge/server/Cargo.toml (add bench section)
- /Users/nick/Documents/dev/FrameForge/server/tests/load_test.sh

**Description**:
Create load testing scripts and performance benchmarks. Use criterion for Rust benchmarks and wrk or k6 for HTTP load testing. Measure request latency (p50, p95, p99), throughput, memory usage under load, and concurrent connection handling. Document baseline performance metrics.

**Acceptance Criteria**:
- [ ] Criterion benchmarks for key functions
- [ ] Load testing script using wrk or k6
- [ ] Tests concurrent requests (10, 50, 100 connections)
- [ ] Measures latency percentiles (p50, p95, p99)
- [ ] Measures requests/second throughput
- [ ] Memory profiling under load
- [ ] Documents baseline performance metrics
- [ ] Comparison with Python backend performance
- [ ] cargo bench passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 3 (Tasks 24-33), Task 40, Task 41 must be completed first

---

## PHASE 5: Testing & Documentation (9 tasks)

### Task 43: Create Integration Test Utilities
**Phase**: 5
**Module**: Testing
**Can run in parallel with**: Task 44
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/tests/common/mod.rs

**Description**:
Create test utilities for integration tests. Implement test server setup, mock AppConfig with test values, test image generation helpers, and HTTP client setup. These utilities will be used by all integration tests.

**Acceptance Criteria**:
- [ ] test_app() function creates test server
- [ ] test_config() creates AppConfig with test values
- [ ] create_test_image() generates valid test image bytes
- [ ] test_client() creates HTTP client for testing
- [ ] Utilities are reusable across tests
- [ ] cargo test --test integration_test compiles
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 44: Write Health Check Integration Test
**Phase**: 5
**Module**: Testing
**Can run in parallel with**: Task 43, Task 45
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/tests/integration_test.rs (partial)

**Description**:
Write integration test for GET /api/health endpoint. Test that server responds with 200 OK and {"status": "ok"} JSON. Verify response format matches Python backend exactly.

**Acceptance Criteria**:
- [ ] Test starts test server
- [ ] Makes GET request to /api/health
- [ ] Asserts 200 OK status
- [ ] Asserts response is JSON
- [ ] Asserts status field equals "ok"
- [ ] Test passes with cargo test
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 45: Write Providers Listing Integration Test
**Phase**: 5
**Module**: Testing
**Can run in parallel with**: Task 43, Task 44, Task 46
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/tests/integration_test.rs (partial)

**Description**:
Write integration test for GET /api/providers endpoint. Test that server responds with list of providers including "google" and "nano-banana". Verify response format matches Python backend.

**Acceptance Criteria**:
- [ ] Test starts test server
- [ ] Makes GET request to /api/providers
- [ ] Asserts 200 OK status
- [ ] Asserts response is JSON array
- [ ] Asserts array contains "google"
- [ ] Asserts array contains "nano-banana"
- [ ] Test passes with cargo test
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 46: Write Edit Endpoint Integration Test
**Phase**: 5
**Module**: Testing
**Can run in parallel with**: Task 43, Task 45
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/tests/integration_test.rs (partial)

**Description**:
Write integration test for POST /api/edit endpoint. Create multipart form with test image and prompt, submit to endpoint, verify response is valid image. Mock AI provider to return predictable result for testing.

**Acceptance Criteria**:
- [ ] Test creates multipart form with image and prompt
- [ ] Makes POST request to /api/edit
- [ ] Asserts 200 OK status (or appropriate status)
- [ ] Asserts response Content-Type is image/*
- [ ] Verifies response body is valid image
- [ ] Tests with mock provider (no API key needed)
- [ ] Test passes with cargo test
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 47: Add Rustdoc Documentation
**Phase**: 5
**Module**: Documentation
**Can run in parallel with**: Task 48, Task 49, Task 50, Task 51
**Files to create/modify**:
- All .rs files (add documentation comments)

**Description**:
Add comprehensive rustdoc comments to all public APIs. Document modules, structs, traits, functions, and important implementation details. Include examples where helpful. This enables `cargo doc` to generate complete API documentation.

**Acceptance Criteria**:
- [ ] All public modules have //! module docs
- [ ] All public structs have /// docs
- [ ] All public functions have /// docs with params and returns
- [ ] Trait methods are documented
- [ ] Examples included for key APIs
- [ ] cargo doc builds without warnings
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 48: Create Deployment Documentation
**Phase**: 5
**Module**: Documentation
**Can run in parallel with**: Task 47, Task 49, Task 50, Task 51
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/README.md

**Description**:
Create comprehensive README.md for the Rust server. Include project description, setup instructions, configuration reference, API documentation links, deployment guide, and troubleshooting tips.

**Acceptance Criteria**:
- [ ] README includes project overview
- [ ] Installation and setup instructions
- [ ] Environment variable reference
- [ ] How to run in development
- [ ] How to build for production
- [ ] API endpoint documentation
- [ ] Troubleshooting section
- [ ] cargo build passes
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 49: Create Dockerfile and Container Configuration
**Phase**: 5
**Module**: Deployment
**Can run in parallel with**: Task 47, Task 48, Task 50, Task 51
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/Dockerfile
- /Users/nick/Documents/dev/FrameForge/server/.dockerignore
- /Users/nick/Documents/dev/FrameForge/server/docker-compose.yml (optional)

**Description**:
Create production-ready Dockerfile using multi-stage build. Use rust:alpine for building and distroless or alpine for runtime. Optimize image size and security. Include docker-compose.yml for local development/testing.

**Acceptance Criteria**:
- [ ] Multi-stage Dockerfile (builder + runtime)
- [ ] Uses rust:alpine or similar for build stage
- [ ] Uses minimal runtime image (distroless, alpine, or scratch)
- [ ] Copies only necessary files to runtime
- [ ] Sets non-root user for security
- [ ] Exposes correct port from config
- [ ] .dockerignore excludes target/, .env, etc.
- [ ] docker-compose.yml for local testing
- [ ] Image builds successfully
- [ ] Container runs and passes health check
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42) must be completed first

---

### Task 50: Create CI/CD Pipeline Configuration
**Phase**: 5
**Module**: Deployment & CI/CD
**Can run in parallel with**: Task 47, Task 48, Task 49, Task 51
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/.github/workflows/rust-ci.yml
- /Users/nick/Documents/dev/FrameForge/.github/workflows/docker-build.yml

**Description**:
Create GitHub Actions workflows for continuous integration and deployment. Run tests on PR, build and push Docker images on merge to main, run linting and security checks. Optionally add deployment automation to staging/production.

**Acceptance Criteria**:
- [ ] CI workflow runs on pull requests
- [ ] Runs cargo test, cargo clippy, cargo fmt --check
- [ ] Runs security audit (cargo audit)
- [ ] Tests pass before merge allowed
- [ ] Docker build workflow on main branch
- [ ] Pushes images to container registry
- [ ] Tags images with commit SHA and latest
- [ ] Includes deployment step (optional)
- [ ] Workflows execute successfully
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42), Task 49 must be completed first

---

### Task 51: Create Performance Comparison Report
**Phase**: 5
**Module**: Testing & Documentation
**Can run in parallel with**: Task 47, Task 48
**Files to create/modify**:
- /Users/nick/Documents/dev/FrameForge/server/docs/performance-comparison.md

**Description**:
Document performance comparison between Rust and Python backends. Run identical load tests on both, measure latency (p50, p95, p99), throughput (requests/second), memory usage, CPU utilization, and startup time. Document findings with charts/tables.

**Acceptance Criteria**:
- [ ] Load tests run on both Rust and Python backends
- [ ] Same test scenarios (health, providers, edit endpoints)
- [ ] Measures latency percentiles (p50, p95, p99)
- [ ] Measures throughput (requests/second)
- [ ] Measures memory usage under load
- [ ] Measures CPU utilization
- [ ] Measures cold start time
- [ ] Documents improvement percentages
- [ ] Includes charts or tables
- [ ] Identifies any regressions or issues
- [ ] Code reviewed by codex

**Dependencies**: Phase 4 (Tasks 34-42), Task 42 must be completed first

---

## Summary

### Task Distribution by Phase
- **Phase 1 (Foundation)**: 10 tasks
- **Phase 2 (Service Layer)**: 13 tasks
- **Phase 3 (API Routes)**: 10 tasks
- **Phase 4 (Middleware & Production)**: 9 tasks
- **Phase 5 (Testing & Deployment)**: 9 tasks

### Parallelization Opportunities

**Phase 1**: Tasks 2, 3, 4 can run in parallel. Tasks 8, 9, 10 can run in parallel.

**Phase 2**: Within each provider (Google: Tasks 13-14, Fal: Tasks 15-20), initial setup tasks can be parallelized. Tasks 21-22 can run in parallel.

**Phase 3**: Tasks 24-25 can run in parallel. Task 26-27 can run in parallel after routes are set up.

**Phase 4**: Tasks 34-39 modify main.rs and require coordination. Tasks 40-42 can run in parallel. Note: main.rs edits should be coordinated to avoid conflicts.

**Phase 5**: Tasks 43-46 can run in parallel. Tasks 47-51 can run in parallel.

### Critical Path
Task 1 → Task 4, 5 → Task 10 → Task 11 → Phase 2 sequential tasks → Task 26 → Task 32 → Task 33 → Phase 4 → Phase 5

### Validation Requirements
Every task requires:
- `cargo build` must pass
- Code review by codex
- Specific acceptance criteria met

### Migration from Rocket to Axum
Note: The existing server uses Rocket framework. The first phase will replace this entirely with Axum, as specified in architecture.md. The existing main.rs will be completely rewritten.
