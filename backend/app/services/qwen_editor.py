from typing import Tuple, Optional, Dict, Any

from .base import ImageEditor


class QwenEditor(ImageEditor):
    async def edit_image(self, image_bytes: bytes, prompt: str, options: Dict[str, Any]) -> Tuple[bytes, Optional[str]]:
        raise NotImplementedError("Qwen Edit provider not implemented yet.")

