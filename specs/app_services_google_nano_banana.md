This file defines an image editing service that uses the Google Gemini API. Here's a breakdown:

### Purpose and Architectural Fit

The file `google_nano_banana.py` contains the `GoogleNanoBananaEditor` class, which is a concrete implementation of the `ImageEditor` service.

The backend is designed using a **Strategy pattern**. `base.py` defines an abstract `ImageEditor` interface, and any class that implements this interface can be used as an image editing service. A factory function in `factory.py` is responsible for selecting the correct editing service (e.g., `'google'`, `'fal'`) based on a request parameter. This design makes the system highly extensible, as you can add new editing providers without changing the core application logic in `main.py`.

### Main Functions

*   `GoogleNanoBananaEditor(settings: Settings)`: The constructor initializes the class, configuring the Google Gemini client using the API key from the application settings.
*   `edit_image(self, image_bytes: bytes, prompt: str) -> bytes`: This is the main method. It takes image data and a text prompt, sends them to the Google Gemini Pro Vision model, and returns the edited image bytes received from the API. It runs the synchronous Google API call in a separate thread using `asyncio.to_thread` to avoid blocking the FastAPI event loop.

### Dependencies

*   **Internal:**
    *   `app.services.base.ImageEditor`: The abstract base class it inherits from.
    *   `app.config.Settings`: The Pydantic settings object used to get the Google API key.
*   **External:**
    *   `google.generativeai`: The Google client library for interacting with the Gemini API.
    *   `PIL` (Pillow): Used for handling and converting image data.
