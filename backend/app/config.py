from functools import lru_cache
from typing import List, Optional

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8", case_sensitive=False)

    google_api_key: Optional[str] = None
    # Default to a current image-capable Gemini model id used in the example
    google_model_id: str = "gemini-2.5-flash-image-preview"

    allowed_origins: List[str] = [
        "http://localhost:5173",
        "http://127.0.0.1:5173",
        "http://localhost:3000",
        "http://127.0.0.1:3000",
    ]


@lru_cache()
def get_settings() -> Settings:
    return Settings()
