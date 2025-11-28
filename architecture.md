# FrameForge Rust Backend Architecture

## Executive Summary

This document outlines the comprehensive architecture for transforming the Python FastAPI backend into a high-performance Rust implementation. The Rust backend will maintain API compatibility while providing improved performance, memory safety, and production reliability.

## Current State Analysis

### Python Backend Overview
The existing Python backend is a FastAPI application providing AI-powered image editing services for virtual home staging. Key components include:

- **Framework**: FastAPI with uvicorn ASGI server
- **Configuration**: Pydantic settings with .env file support
- **Service Layer**: Abstract base class pattern with multiple AI provider implementations
  - Google Gemini (Nano Banana) - Default provider
  - Fal.ai dynamic model support
- **API Endpoints**:
  - `GET /api/health` - Health check
  - `GET /api/providers` - List available AI providers
  - `POST /api/edit` - Multi-image upload with AI-powered editing
- **Features**: CORS support, multipart file uploads, streaming responses

### Existing Rust Server
Located at `/Users/nick/Documents/dev/FrameForge/server/`, currently contains:
- Minimal Rocket framework setup (v0.5.1)
- Basic health check endpoint
- Edition 2024 Rust configuration

## Recommended Technology Stack

### Core Framework: Axum
**Chosen Package**: `axum = "0.8"`

**Justification**:
- Built on Tokio runtime with native async/await support
- Excellent balance of performance and ergonomics
- Type-safe routing and extractors
- Tower middleware ecosystem integration
- Production-ready with strong community support
- Better suited for modern async patterns than Rocket
- Close performance to Actix-web with simpler learning curve

**Sources**: [Rust Web Frameworks Compared](https://dev.to/leapcell/rust-web-frameworks-compared-actix-vs-axum-vs-rocket-4bad), [Rust Web Framework Showdown](https://dev.to/lamri_abdellahramdane_15/rust-web-framework-showdown-actix-vs-axum-vs-rocket-stop-obsessing-over-benchmarks-2l5l)

### Async Runtime: Tokio
**Chosen Package**: `tokio = { version = "1.48", features = ["full"] }`

**Justification**:
- Industry standard async runtime (437M+ downloads)
- Zero-cost abstractions with bare-metal performance
- Comprehensive I/O driver (epoll, kqueue, IOCP)
- Work-stealing multi-threaded scheduler
- Required by Axum framework

**Sources**: [Tokio Complete Guide](https://generalistprogrammer.com/tutorials/tokio-rust-crate-guide), [Async Rust State](https://corrode.dev/blog/async/)

### HTTP Client: reqwest
**Chosen Package**: `reqwest = { version = "0.12", features = ["json", "multipart", "stream"] }`

**Justification**:
- Higher-level than hyper with ergonomic API
- Built-in async support with Tokio integration
- Handles cookies, redirects, proxies automatically
- Perfect for downloading images from AI providers
- Production-ready and widely adopted

**Sources**: [Best Rust HTTP Client](https://blog.logrocket.com/best-rust-http-client/), [reqwest GitHub](https://github.com/seanmonstar/reqwest)

### Serialization: serde & serde_json
**Chosen Packages**:
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Justification**:
- De facto standard for Rust serialization
- 500-1000 MB/s deserialization performance
- Type-safe with compile-time guarantees
- Extensive derive macro support
- Required for Axum JSON responses

**Sources**: [serde_json Guide](https://generalistprogrammer.com/tutorials/serde_json-rust-crate-guide), [Serde Best Practices](https://dev.to/aaravjoshi/how-serde-transforms-rust-data-serialization-complete-performance-and-safety-guide-36f2)

### Configuration Management: config & dotenvy
**Chosen Packages**:
```toml
config = "0.14"
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
```

**Justification**:
- `dotenvy`: Well-maintained fork of dotenv (original unmaintained since 2020)
- Type-safe configuration structs with serde
- Environment variable support with .env files
- Flexible configuration sources (files, env, defaults)

**Sources**: [dotenvy docs](https://docs.rs/dotenvy/latest/dotenvy/), [Load .env in Rust](https://pyk.sh/blog/2024-01-07-load-env-in-rust)

### Image Processing: image
**Chosen Package**: `image = "0.25"`

**Justification**:
- Native Rust implementation (PIL/Pillow equivalent)
- Supports loading/saving from bytes
- Multiple format support (PNG, JPEG, WebP, etc.)
- Memory-efficient streaming support
- Type-safe image operations

**Sources**: [image crate](https://crates.io/crates/image), [GitHub image-rs/image](https://github.com/image-rs/image)

### Base64 Encoding: base64
**Chosen Package**: `base64 = "0.22"`

**Justification**:
- Fast and correct base64 implementation
- URL-safe encoding support
- Required for data URL generation
- Zero-copy operations where possible

**Sources**: [base64 docs](https://docs.rs/base64), [Base64 with Rust Guide](https://mojoauth.com/binary-encoding-decoding/base64-with-rust/)

### CORS Middleware: tower-http
**Chosen Package**: `tower-http = { version = "0.6", features = ["cors", "trace", "compression"] }`

**Justification**:
- Official Tower middleware collection
- Seamless Axum integration
- Production-ready CORS handling
- Additional features: compression, tracing, request ID

**Sources**: [CORS in Axum Guide](https://www.ruststepbystep.com/how-to-handle-cors-in-rust-with-axum-a-step-by-step-guide/), [tower-http CORS docs](https://docs.rs/tower-http/latest/tower_http/cors/index.html)

### Error Handling: anyhow & thiserror
**Chosen Packages**:
```toml
anyhow = "2.0"
thiserror = "2.0"
```

**Justification**:
- `thiserror`: Define custom error types for API boundaries
- `anyhow`: Application-level error propagation with context
- Hybrid approach: thiserror for domain errors, anyhow for infrastructure
- Best practice for web APIs in 2025

**Sources**: [Rust Error Handling Guide 2025](https://markaicode.com/rust-error-handling-2025-guide/), [anyhow vs thiserror](https://www.shakacode.com/blog/thiserror-anyhow-or-how-i-handle-errors-in-rust-apps/)

### AI Provider SDKs

#### Google Gemini
**Chosen Package**: `google-generativeai = "0.7"` OR `rust-genai = "0.1"`

**Justification**:
- `google-generativeai`: Dedicated Gemini client
- `rust-genai`: Multi-provider SDK (supports Gemini, OpenAI, Anthropic, etc.)
- Async/await support with Tokio
- Type-safe API interactions

**Recommendation**: Start with `rust-genai` for multi-provider support, easier to add more AI providers later.

**Sources**: [rust-genai GitHub](https://github.com/jeremychone/rust-genai), [gemini_client_rs](https://lib.rs/crates/gemini_client_rs)

#### Fal.ai
**Package**: Custom HTTP client implementation using `reqwest`

**Justification**:
- No official Rust SDK yet
- RESTful API works well with reqwest
- Dynamic model path support via HTTP calls
- Maintains Python backend's flexibility

### Multipart Form Handling: axum-typed-multipart
**Chosen Package**: `axum-typed-multipart = "0.12"`

**Justification**:
- Type-safe multipart/form-data handling
- `FieldData<NamedTempFile>` for image uploads
- Configurable size limits
- Better than manual multipart parsing

**Sources**: [axum Multipart docs](https://docs.rs/axum/latest/axum/extract/struct.Multipart.html), [axum_typed_multipart docs](https://docs.rs/axum_typed_multipart)

### Logging & Tracing: tracing & tracing-subscriber
**Chosen Packages**:
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

**Justification**:
- Async-aware structured logging
- Tower middleware integration
- Production-ready observability
- JSON output for log aggregation

## Complete Cargo.toml

```toml
[package]
name = "frameforge-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = { version = "0.8", features = ["multipart", "macros"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace", "compression", "fs"] }

# Async runtime
tokio = { version = "1.48", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Configuration
config = "0.14"
dotenvy = "0.15"

# HTTP client
reqwest = { version = "0.12", features = ["json", "multipart", "stream"] }

# Image processing
image = "0.25"
base64 = "0.22"

# Multipart handling
axum-typed-multipart = "0.12"

# Error handling
anyhow = "2.0"
thiserror = "2.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# AI providers
rust-genai = "0.1"  # Multi-provider AI client

# Utilities
mime = "0.3"
bytes = "1.9"

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
```

## Project Structure

```
server/
├── Cargo.toml
├── Cargo.lock
├── .env.example
├── src/
│   ├── main.rs              # Application entry point, Axum server setup
│   ├── lib.rs               # Library root (optional, for testing)
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Custom error types with thiserror
│   ├── routes/
│   │   ├── mod.rs           # Route module exports
│   │   ├── health.rs        # Health check endpoint
│   │   ├── providers.rs     # List providers endpoint
│   │   └── edit.rs          # Image editing endpoint
│   ├── services/
│   │   ├── mod.rs           # Service module exports
│   │   ├── base.rs          # ImageEditor trait definition
│   │   ├── factory.rs       # Service factory pattern
│   │   ├── google_nano_banana.rs  # Google Gemini implementation
│   │   └── fal_editor.rs    # Fal.ai implementation
│   ├── models/
│   │   ├── mod.rs           # Model exports
│   │   ├── request.rs       # Request DTOs
│   │   └── response.rs      # Response DTOs
│   └── utils/
│       ├── mod.rs           # Utility exports
│       └── image_utils.rs   # Image processing utilities
└── tests/
    ├── integration_test.rs  # Integration tests
    └── common/
        └── mod.rs           # Test utilities
```

## Module Breakdown

### 1. main.rs
**Responsibilities**:
- Initialize tracing/logging
- Load configuration
- Set up Axum router with routes
- Configure CORS middleware
- Start Tokio runtime and bind server

**Key Components**:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    // Load config
    // Build router
    // Add middleware (CORS, tracing, compression)
    // Bind and serve
}
```

### 2. config.rs
**Responsibilities**:
- Define `AppConfig` struct with serde
- Load from .env file using dotenvy
- Provide typed access to settings
- Validate configuration at startup

**Key Fields**:
- `google_api_key: Option<String>`
- `gemini_api_key: Option<String>`
- `fal_key: Option<String>`
- `google_model_id: String` (default)
- `allowed_origins: Vec<String>`
- `server_port: u16`
- `server_host: String`

### 3. error.rs
**Responsibilities**:
- Define custom error types with thiserror
- Implement `IntoResponse` for Axum error handling
- Provide user-friendly error messages
- Log errors with tracing

**Error Types**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid image format")]
    InvalidImageFormat,

    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("AI service error: {0}")]
    AiServiceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}
```

### 4. routes/health.rs
**Endpoint**: `GET /api/health`

**Response**:
```json
{
  "status": "ok"
}
```

### 5. routes/providers.rs
**Endpoint**: `GET /api/providers`

**Response**:
```json
["google", "nano-banana", "fal:fal-ai/flux/dev", ...]
```

**Logic**:
- Return static providers from factory
- Include dynamic fal: prefixed options

### 6. routes/edit.rs
**Endpoint**: `POST /api/edit`

**Request**:
- Multipart form data
- Multiple image files
- Text prompt field
- Optional provider field
- Optional API keys in headers

**Response**:
- Streaming image response
- Content-Type: image/png (or appropriate MIME type)

**Logic**:
1. Extract multipart form data
2. Validate uploaded files are images
3. Get provider from request (default to google)
4. Extract API keys from headers if present
5. Use factory to get appropriate service
6. Call service.edit_image()
7. Stream response back to client

### 7. services/base.rs
**Trait Definition**:
```rust
#[async_trait]
pub trait ImageEditor: Send + Sync {
    async fn edit_image(
        &self,
        images: Vec<bytes::Bytes>,
        prompt: String,
        options: HashMap<String, String>,
    ) -> Result<(bytes::Bytes, Option<String>), ApiError>;
}
```

### 8. services/factory.rs
**Responsibilities**:
- Implement `get_editor()` function
- Handle provider name matching
- Support dynamic `fal:` prefix parsing
- Default to GoogleNanoBananaEditor

**Logic**:
```rust
pub fn get_editor(
    provider_name: &str,
    config: &AppConfig,
    runtime_api_keys: HashMap<String, String>,
) -> Result<Box<dyn ImageEditor>, ApiError> {
    // Handle fal: prefix
    // Match static providers
    // Default to google
}
```

### 9. services/google_nano_banana.rs
**Implementation**: Google Gemini AI image editing

**Dependencies**:
- `rust-genai` crate
- API key from config or runtime

**Logic**:
1. Initialize Gemini client
2. Convert images to base64 or API format
3. Call Gemini API with prompt
4. Handle streaming response
5. Return edited image bytes

**Fallback**: Return original image if no API key (development mode)

### 10. services/fal_editor.rs
**Implementation**: Fal.ai dynamic model support

**Dependencies**:
- `reqwest` for HTTP calls
- Model path from provider string

**Logic**:
1. Parse model path from `fal:` prefix
2. Upload images to Fal.ai
3. Submit generation request
4. Poll for completion
5. Download result image
6. Return image bytes

## Migration Strategy

### Phase 1: Foundation (Week 1)
1. Set up new Cargo.toml with all dependencies
2. Implement config.rs and error.rs
3. Create basic Axum server in main.rs
4. Add CORS middleware
5. Implement health check endpoint
6. Set up tracing/logging

**Milestone**: Server starts and responds to health checks

### Phase 2: Service Layer (Week 2)
1. Implement base.rs trait
2. Create factory.rs with provider mapping
3. Implement GoogleNanoBananaEditor
4. Test Google Gemini integration
5. Implement FalEditor
6. Test Fal.ai integration

**Milestone**: Both AI providers work in isolation

### Phase 3: API Endpoints (Week 3)
1. Implement providers endpoint
2. Implement edit endpoint with multipart handling
3. Add image validation
4. Implement streaming responses
5. Add error handling
6. Test with real images

**Milestone**: All endpoints functional

### Phase 4: Testing & Optimization (Week 4)
1. Write integration tests
2. Load testing with concurrent requests
3. Memory profiling
4. Add request timeout handling
5. Optimize image processing
6. Documentation

**Milestone**: Production-ready

### Phase 5: Deployment (Week 5)
1. Create Dockerfile
2. Set up CI/CD pipeline
3. Deploy to staging
4. Performance comparison with Python
5. Fix any issues
6. Production deployment

**Milestone**: Live in production

## Dependency Relationships

```
main.rs
  ├─> config.rs (load settings)
  ├─> routes/* (register endpoints)
  └─> middleware (CORS, tracing)

routes/edit.rs
  ├─> services/factory.rs (get editor)
  ├─> models/request.rs (parse input)
  ├─> models/response.rs (format output)
  └─> error.rs (error handling)

services/factory.rs
  ├─> services/base.rs (trait)
  ├─> services/google_nano_banana.rs
  └─> services/fal_editor.rs

services/google_nano_banana.rs
  ├─> services/base.rs (impl trait)
  ├─> config.rs (API keys)
  ├─> utils/image_utils.rs (image processing)
  └─> error.rs (error types)

services/fal_editor.rs
  ├─> services/base.rs (impl trait)
  ├─> config.rs (API keys)
  ├─> utils/image_utils.rs (image processing)
  └─> error.rs (error types)
```

## Key Design Patterns

### 1. Factory Pattern
The `factory.rs` module implements the factory pattern to abstract service creation, allowing dynamic provider selection without coupling the API layer to specific implementations.

### 2. Strategy Pattern
The `ImageEditor` trait defines a common interface, and each provider implements this strategy differently, enabling interchangeable algorithms.

### 3. Dependency Injection
Configuration and runtime parameters are injected into services, making the code testable and flexible.

### 4. Error Context Enrichment
Using `anyhow::Context`, errors are enriched with additional information as they bubble up through layers.

### 5. Type-Safe Extractors
Axum's extractors provide compile-time safety for request parsing, eliminating runtime validation errors.

## Potential Challenges & Solutions

### Challenge 1: Async Gemini SDK
**Issue**: Some Gemini SDKs may be synchronous.

**Solution**:
- Use `tokio::task::spawn_blocking` for sync code
- Or implement custom async client using reqwest
- rust-genai crate provides async interface

### Challenge 2: Streaming Large Images
**Issue**: Large images may consume significant memory.

**Solution**:
- Use `bytes::Bytes` for zero-copy operations
- Implement streaming uploads/downloads with reqwest
- Set appropriate body size limits in Axum
- Use `tower-http` compression

### Challenge 3: Multi-Image Upload Handling
**Issue**: Processing multiple images efficiently.

**Solution**:
- Use `axum-typed-multipart` for type-safe handling
- Process images concurrently with `tokio::spawn`
- Set reasonable upload limits
- Validate image formats early

### Challenge 4: API Key Management
**Issue**: Supporting both config file and per-request API keys.

**Solution**:
- Accept API keys in request headers
- Fall back to config file values
- Use `Option<String>` for flexible handling
- Validate keys before service instantiation

### Challenge 5: Error Response Compatibility
**Issue**: Maintaining API compatibility with Python backend.

**Solution**:
- Implement custom `IntoResponse` for error types
- Return consistent JSON error format
- Match HTTP status codes
- Include error context in development mode

### Challenge 6: Testing AI Services
**Issue**: Testing without consuming API credits.

**Solution**:
- Mock trait implementations for testing
- Use development mode fallback (return original image)
- Record/replay HTTP interactions
- Integration tests with test API keys

### Challenge 7: CORS Configuration
**Issue**: Matching Python backend's CORS settings.

**Solution**:
- Use `tower-http::cors::CorsLayer`
- Load allowed origins from config
- Support wildcard in development
- Restrict in production

## Performance Expectations

### Latency Improvements
- **Request handling**: 50-70% faster than Python (native async)
- **Image processing**: 30-50% faster (zero-copy operations)
- **Memory usage**: 40-60% reduction (no GC overhead)
- **Throughput**: 2-3x more requests/second

### Concurrency Benefits
- Lightweight tasks (not OS threads)
- Work-stealing scheduler
- Better CPU utilization
- Lower memory per connection

### Production Metrics to Monitor
- Request latency (p50, p95, p99)
- Memory usage per connection
- CPU utilization
- Error rates
- AI provider API latency
- Image processing time

## Security Considerations

### Input Validation
- Validate file types before processing
- Size limits on uploads (via Axum config)
- Sanitize file names
- Rate limiting per IP

### API Key Protection
- Never log API keys
- Use `dotenvy` for local development
- Environment variables in production
- Support per-request key override for multi-tenant

### CORS Security
- Restrict allowed origins in production
- Validate origin headers
- No credentials in CORS for public API

### Error Information Disclosure
- Generic errors to clients
- Detailed logs server-side only
- No stack traces in production

## Testing Strategy

### Unit Tests
- Test each service implementation independently
- Mock `ImageEditor` trait for factory tests
- Test error conversions
- Test configuration loading

### Integration Tests
- Test full request/response cycle
- Test with real file uploads
- Test provider selection logic
- Test error responses

### Load Tests
- Concurrent request handling
- Large file uploads
- Memory leak detection
- Connection pool exhaustion

### Compatibility Tests
- API compatibility with Python version
- Same response formats
- Same error codes
- Frontend integration

## Documentation Requirements

### Code Documentation
- Rustdoc comments on all public APIs
- Examples in trait documentation
- Configuration field descriptions
- Error variant explanations

### API Documentation
- OpenAPI/Swagger spec generation
- Request/response examples
- Error code reference
- Authentication guide

### Deployment Documentation
- Environment variable reference
- Docker deployment guide
- Production configuration examples
- Monitoring setup

## Future Enhancements

### Short Term
1. Add more AI providers (OpenAI DALL-E, Stability AI)
2. Implement request caching
3. Add webhook support for async processing
4. Metrics endpoint (Prometheus format)

### Medium Term
1. WebSocket support for real-time updates
2. Batch processing API
3. Image storage integration (S3, GCS)
4. User authentication/authorization

### Long Term
1. gRPC API for internal services
2. Distributed tracing
3. Multi-region deployment
4. Edge computing support

## Migration Validation Checklist

- [ ] All Python endpoints have Rust equivalents
- [ ] Response formats match exactly
- [ ] Error codes match
- [ ] CORS configuration matches
- [ ] API key handling matches
- [ ] Multi-image upload works
- [ ] Google Gemini integration works
- [ ] Fal.ai integration works
- [ ] Health check works
- [ ] Provider listing works
- [ ] Default prompts match
- [ ] Image streaming works
- [ ] Performance metrics collected
- [ ] Load tests pass
- [ ] Integration tests pass
- [ ] Frontend connects successfully
- [ ] Production deployment ready

## Conclusion

This architecture provides a robust foundation for migrating the Python FastAPI backend to Rust. The chosen technology stack balances performance, developer ergonomics, and production readiness. The modular design allows for incremental migration and testing, reducing deployment risk.

The Axum + Tokio combination provides excellent async performance while maintaining code clarity. The service layer architecture with trait-based abstractions ensures extensibility for adding new AI providers. Error handling with thiserror and anyhow follows Rust best practices for 2025.

With careful execution of the migration strategy, the Rust backend will deliver significant performance improvements while maintaining full API compatibility with the existing Python implementation.

---

**Document Version**: 1.0
**Created**: November 27, 2025
**Author**: Architecture Analysis Agent
**Status**: Ready for Implementation
