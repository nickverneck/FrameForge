from typing import List

from ..config import Settings
from .base import ImageEditor
from .google_nano_banana import GoogleNanoBananaEditor
from .fal_editor import FalImageEditor


PROVIDERS = {
    "google": GoogleNanoBananaEditor,
    "nano-banana": GoogleNanoBananaEditor,  # alias for google
    # FAL providers are dynamic via prefix 'fal:' followed by model path
}


def list_providers() -> List[str]:
    # We only list static keys here; dynamic FAL providers are not enumerated
    return sorted(set(PROVIDERS.keys()))


def get_editor(provider_name: str, settings: Settings) -> ImageEditor:
    name = (provider_name or "google").strip()

    # Handle FAL providers of the form 'fal:<model_path>'
    if name.lower().startswith("fal:"):
        model_path = name.split(":", 1)[1]
        return FalImageEditor(model_path=model_path, settings=settings)

    # Otherwise map to known providers
    key = name.lower()
    cls = PROVIDERS.get(key, GoogleNanoBananaEditor)
    return cls(settings) if cls is GoogleNanoBananaEditor else cls()
