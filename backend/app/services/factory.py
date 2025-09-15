from typing import List

from ..config import Settings
from .base import ImageEditor
from .google_nano_banana import GoogleNanoBananaEditor
from .qwen_editor import QwenEditor
from .kontext_pro_editor import KontextProEditor


PROVIDERS = {
    "google": GoogleNanoBananaEditor,
    "nano-banana": GoogleNanoBananaEditor,  # alias
    "qwen": QwenEditor,
    "kontext": KontextProEditor,
}


def list_providers() -> List[str]:
    return sorted(set(PROVIDERS.keys()))


def get_editor(provider_name: str, settings: Settings) -> ImageEditor:
    key = (provider_name or "google").lower()
    cls = PROVIDERS.get(key)
    if not cls:
        # default to google if unknown
        cls = GoogleNanoBananaEditor
    return cls(settings) if cls is GoogleNanoBananaEditor else cls()

