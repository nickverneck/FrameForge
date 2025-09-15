from typing import Tuple, Optional, Dict, Any
import os
import asyncio

import logging
from .base import ImageEditor
from ..config import Settings


def _guess_mime(data: bytes) -> str:
    if not data:
        return "application/octet-stream"
    if data.startswith(b"\xff\xd8\xff"):
        return "image/jpeg"
    if data.startswith(b"\x89PNG\r\n\x1a\n"):
        return "image/png"
    if data.startswith(b"GIF87a") or data.startswith(b"GIF89a"):
        return "image/gif"
    if len(data) > 12 and data[:4] == b"RIFF" and data[8:12] == b"WEBP":
        return "image/webp"
    return "application/octet-stream"


class GoogleNanoBananaEditor(ImageEditor):
    """Google Gemini Flash image editing (aka nano-banana) via google-genai SDK.

    Uses streaming generation to receive an edited image as inline_data.
    If the SDK or API key are not present, returns the original image for dev.
    """

    def __init__(self, settings: Settings):
        # Support both GOOGLE_API_KEY and GEMINI_API_KEY envs
        self.api_key = (
            settings.google_api_key
            or settings.gemini_api_key
            or os.getenv("GOOGLE_API_KEY")
            or os.getenv("GEMINI_API_KEY")
        )
        self.model_id = settings.google_model_id or os.getenv(
            "GOOGLE_MODEL_ID", "gemini-2.5-flash-image-preview"
        )
        self._log = logging.getLogger("frameforge.google")

    async def edit_image(
        self, image_bytes: bytes, prompt: str, options: Dict[str, Any]
    ) -> Tuple[bytes, Optional[str]]:
        if not self.api_key:
            # Dev fallback: no API key, return original
            self._log.warning("Google provider fallback: no API key found; returning original image.")
            return image_bytes, None

        # Run the synchronous google-genai client in a thread
        def _run_sync() -> Tuple[bytes, Optional[str]]:
            try:
                from google import genai
                from google.genai import types
            except Exception as e:  # ImportError or others
                raise RuntimeError(
                    "google-genai is not installed. Add it to dependencies and install."
                ) from e

            client = genai.Client(api_key=self.api_key)

            # Prepare contents: include the input image and the instruction text
            input_mime = _guess_mime(image_bytes)
            contents = [
                types.Content(
                    role="user",
                    parts=[
                        types.Part.from_bytes(data=image_bytes, mime_type=input_mime),
                        types.Part.from_text(text=prompt or ""),
                    ],
                )
            ]

            config = types.GenerateContentConfig(
                response_modalities=["IMAGE"],
            )

            last_image_bytes: Optional[bytes] = None
            last_image_mime: Optional[str] = None

            for chunk in client.models.generate_content_stream(
                model=self.model_id, contents=contents, config=config
            ):
                # Each chunk may contain text or inline image data
                if (
                    not getattr(chunk, "candidates", None)
                    or not chunk.candidates
                    or not getattr(chunk.candidates[0], "content", None)
                    or not getattr(chunk.candidates[0].content, "parts", None)
                ):
                    continue

                parts = chunk.candidates[0].content.parts
                for p in parts:
                    inline = getattr(p, "inline_data", None)
                    if inline and getattr(inline, "data", None):
                        last_image_bytes = inline.data
                        last_image_mime = getattr(inline, "mime_type", None)

            if last_image_bytes is None:
                raise RuntimeError("No edited image returned from Gemini stream.")

            return last_image_bytes, last_image_mime

        return await asyncio.to_thread(_run_sync)
