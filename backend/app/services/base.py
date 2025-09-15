from abc import ABC, abstractmethod
from typing import Tuple, Optional, Dict, Any


class ImageEditor(ABC):
    @abstractmethod
    async def edit_image(self, image_bytes: bytes, prompt: str, options: Dict[str, Any]) -> Tuple[bytes, Optional[str]]:
        """
        Returns: (edited_image_bytes, mime_type)
        mime_type may be None to reuse input mime.
        """
        raise NotImplementedError

