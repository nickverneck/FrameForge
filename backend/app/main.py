from fastapi import FastAPI, UploadFile, File, Form, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import StreamingResponse, JSONResponse
from typing import Optional
import logging

from .config import get_settings
from .services.factory import get_editor, list_providers


settings = get_settings()

app = FastAPI(title="FrameForge API", version="0.1.0")
logger = logging.getLogger("frameforge")

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.allowed_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/api/health")
def health():
    return {"status": "ok"}


@app.get("/api/providers")
def providers():
    return {"default": "google", "available": list_providers()}


@app.post("/api/edit")
async def edit_image(
    image: UploadFile = File(..., description="Unfurnished room photo"),
    prompt: Optional[str] = Form(None, description="Prompt or style instructions"),
    provider: Optional[str] = Form(None, description="Provider: google|qwen|kontext"),
):
    if image.content_type is None or not image.content_type.startswith("image/"):
        raise HTTPException(status_code=400, detail="Please upload a valid image file.")

    raw = await image.read()
    if not raw:
        raise HTTPException(status_code=400, detail="Uploaded image is empty.")

    # Default prompt template if not provided
    if not prompt:
        prompt = (
            "Stage this room with minimalist modern furniture in neutral tones. "
            "Preserve architecture and lighting; add realistic shadows and reflections."
        )

    provider_name = provider or "google"
    logger.info(f"Editing image via provider='{provider_name}'")
    editor = get_editor(provider_name=provider_name, settings=settings)

    try:
        edited_bytes, mime = await editor.edit_image(raw, prompt=prompt, options={})
    except NotImplementedError as e:
        raise HTTPException(status_code=501, detail=str(e))
    except Exception as e:
        logger.exception("Editing failed")
        raise HTTPException(status_code=500, detail=f"Editing failed: {e}")

    media_type = mime or image.content_type
    logger.info(f"Edit complete provider='{provider_name}' content_type='{media_type}' bytes={len(edited_bytes)}")
    return StreamingResponse(iter([edited_bytes]), media_type=media_type)
