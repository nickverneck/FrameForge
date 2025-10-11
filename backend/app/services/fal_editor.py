import asyncio
import logging
from typing import Any, Dict, Optional, Tuple, List

import os
import base64

from .base import ImageEditor
from ..config import Settings


class FalImageEditor(ImageEditor):
    """Generic FAL.ai image editor using the fal-client SDK.

    This class is reusable across different FAL-served models. Instantiate it
    with a model path like:
      - "fal-ai/nano-banana/edit"
      - "fal-ai/qwen-image-edit"
      - "fal-ai/bytedance/seedream/v4/edit"
      - "fal-ai/flux-kontext/dev"

    Env: FAL_KEY must be set.
    """

    def __init__(self, model_path: str, settings: Settings | None = None):
        self.model_path = model_path
        # Prefer Settings.fal_key, then env var
        self.api_key = (settings.fal_key if settings else None) or os.getenv("FAL_KEY")
        self._log = logging.getLogger("frameforge.fal")

    async def edit_image(self, image_bytes: List[bytes], prompt: str, options: Dict[str, Any]) -> Tuple[bytes, Optional[str]]:
        if not self.api_key:
            self._log.warning("FAL provider fallback: FAL_KEY missing; returning original image.")
            return image_bytes[0], None

        def _run_sync() -> Tuple[bytes, Optional[str]]:
            try:
                import fal_client
            except Exception as e:
                raise RuntimeError("fal-client is not installed. Add 'fal-client' to dependencies and install.") from e

            # Ensure SDK sees the key
            if self.api_key and not os.getenv("FAL_KEY"):
                os.environ["FAL_KEY"] = self.api_key

            data_uris = []
            for img_bytes in image_bytes:
                mime = "image/jpeg"
                if img_bytes.startswith(b"\x89PNG\r\n\x1a\n"):
                    mime = "image/png"
                elif img_bytes.startswith(b"\xff\xd8\xff"):
                    mime = "image/jpeg"
                elif img_bytes.startswith(b"GIF87a") or img_bytes.startswith(b"GIF89a"):
                    mime = "image/gif"
                data_uris.append(f"data:{mime};base64,{base64.b64encode(img_bytes).decode('ascii')}")

            # Specific models have different arg names
            if "flux-kontext" in self.model_path or "qwen-image-edit" in self.model_path:
                args = {
                    "prompt": prompt,
                    "image_url": data_uris[0],
                    "output_format": "png",
                    "sync_mode": True,
                }
            else:
                args = {
                    "prompt": prompt,
                    "image_urls": data_uris,
                    "output_format": "png",
                    # When supported, FAL returns data URIs directly if sync_mode is True
                    # This may not be honored by all models; we still handle HTTP URLs below.
                    "sync_mode": True,
                }

            # Prefer subscribe() which returns the result when complete
            try:
                result = fal_client.subscribe(
                    self.model_path,
                    arguments=args,
                    with_logs=False,
                )
            except Exception as e:
                raise RuntimeError(f"FAL model invocation failed: {e}")

            # Extract image URL(s)
            candidate_urls = []
            if isinstance(result, dict):
                if "images" in result and isinstance(result["images"], list) and result["images"]:
                    first = result["images"][0]
                    url = first.get("url") if isinstance(first, dict) else None
                    if url:
                        candidate_urls.append(url)
                if "image" in result and isinstance(result["image"], dict):
                    url = result["image"].get("url")
                    if url:
                        candidate_urls.append(url)
                if "result" in result and isinstance(result["result"], dict):
                    url = result["result"].get("url")
                    if url:
                        candidate_urls.append(url)

            if candidate_urls:
                url = candidate_urls[0]
                # If it's a data URI, decode locally
                if url.startswith("data:"):
                    try:
                        header, b64 = url.split(",", 1)
                        mime = "image/png"
                        if header.startswith("data:") and ";base64" in header:
                            mime = header[5: header.index(";")]
                        data = base64.b64decode(b64)
                        return data, mime
                    except Exception as e:
                        self._log.warning(f"Failed to decode data URI from FAL: {e}")
                # Otherwise fetch over HTTP(S)
                try:
                    import httpx
                    with httpx.Client(timeout=120) as client:
                        resp = client.get(url)
                        resp.raise_for_status()
                        mime = resp.headers.get("content-type")
                        return resp.content, mime
                except Exception as e:
                    self._log.warning(f"Failed to fetch FAL image URL: {e}")

            return image_bytes[0], None

        return await asyncio.to_thread(_run_sync)
