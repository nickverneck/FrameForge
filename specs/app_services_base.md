This file, `base.py`, defines the core abstraction for image editing services within the application.

*   **Purpose:** It establishes a standardized interface for any image editing service. By defining an abstract base class (ABC) called `ImageEditor`, it ensures that all concrete image editor implementations will have a consistent set of methods.

*   **Main Components:**
    *   **`ImageEditor` (class):** An abstract base class that serves as a template for image editors.
    *   **`edit_image` (abstract method):** A required asynchronous method that all subclasses of `ImageEditor` must implement. It is designed to take raw image data (`List[bytes]`), a text `prompt`, and a dictionary of `options`, and it returns the edited image bytes and an optional MIME type.

*   **Dependencies:** The file uses Python's built-in `abc` module for creating the abstract class and the `typing` module for type hints. It has no external library dependencies.

*   **Architectural Role:** This file is foundational to the service layer of the backend. It decouples the main application logic from the specific details of any particular image editing API or library. Other services in the `services` directory (like `fal_editor.py` or `google_nano_banana.py`) are expected to inherit from `ImageEditor` and provide concrete implementations of the `edit_image` method. This design allows for different editing services to be used interchangeably, likely chosen by a factory or configuration.
