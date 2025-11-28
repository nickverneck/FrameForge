This file, `/Users/nick/Documents/dev/FrameForge/backend/app/main.py`, is the heart of the backend application, acting as the main entry point for all API requests. It is built using the **FastAPI** web framework.

### Purpose and Functionality

The primary purpose of this file is to define the API endpoints for the "FrameForge" service, which appears to be an AI-powered image editing tool. Specifically, it's designed for **virtual home staging**: taking images of unfurnished rooms and adding realistic furniture based on a text prompt.

### Main Endpoints

1.  **`GET /api/health`**: A standard health check endpoint. It simply returns `{"status": "ok"}` to indicate that the API is running and responsive. This is commonly used for monitoring.
2.  **`GET /api/providers`**: This endpoint informs the client application about the available AI image editing services (e.g., `google`, `qwen`). It allows the frontend to present different options to the user.
3.  **`POST /api/edit`**: This is the core functional endpoint of the API.
    *   It accepts a list of uploaded images and a text `prompt` from the user.
    *   If no prompt is provided, it uses a default prompt specifically designed for staging a room with "minimalist modern furniture."
    *   It validates that the uploaded files are indeed images.
    *   It dynamically selects an image editing service (the "provider") based on the user's choice.
    *   It securely handles API keys passed in the request headers, allowing for per-user or per-request authentication with the underlying AI provider.
    *   It calls the appropriate service to perform the image editing and streams the final image back to the user.

### Dependencies

*   **External**: The main dependency is `fastapi`, a modern, high-performance Python web framework.
*   **Internal**:
    *   `app.config`: This module is used to manage application settings, such as the list of allowed origins for CORS (Cross-Origin Resource Sharing) and API keys.
    *   `app.services.factory`: This module contains a factory pattern (`get_editor`). This is a crucial part of the design that allows the application to be modular. It can easily be extended to support new AI image editing providers without changing the main API logic in this file.

### Role in the Architecture

This file serves as the **controller layer** of the backend. It handles the API routing, request validation, and orchestration. It defines the public interface of the backend service and delegates the complex business logic (the actual image manipulation) to a separate **service layer** (the "editors" obtained from the factory).

This clean separation of concerns makes the application robust and maintainable. The API endpoints can evolve independently of the underlying implementation of the image editing services. The use of CORS middleware indicates it's specifically designed to be called by a separate frontend application, which aligns with the project's overall `frontend` and `backend` directory structure.
