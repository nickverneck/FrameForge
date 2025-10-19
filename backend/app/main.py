from fastapi import FastAPI, UploadFile, File, Form, HTTPException, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import StreamingResponse, JSONResponse
from typing import Optional, List
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
    request: Request,
    images: List[UploadFile] = File(..., description="Unfurnished room photos"),
    prompt: Optional[str] = Form(None, description="Prompt or style instructions"),
    provider: Optional[str] = Form(None, description="Provider: google|qwen|kontext"),
):
    for image in images:
        if image.content_type is None or not image.content_type.startswith("image/"):
            raise HTTPException(status_code=400, detail="Please upload valid image files.")

    raw_images = [await image.read() for image in images]
    if not all(raw_images):
        raise HTTPException(status_code=400, detail="Uploaded image is empty.")

    # Default prompt template if not provided
    if not prompt:
        prompt = (
            "Stage this room with minimalist modern furniture in neutral tones. "
            "Preserve architecture and lighting; add realistic shadows and reflections."
        )

    provider_name = provider or "google"
    request_settings = settings
    auth_header = request.headers.get("authorization")
    if auth_header:
        token = auth_header.strip()
        for prefix in ("bearer ", "key "):
            if token.lower().startswith(prefix):
                token = token[len(prefix):].strip()
                break
        if token:
            request_settings = settings.model_copy(update={"fal_key": token})

    logger.info(f"Editing image via provider='{provider_name}'")
    editor = get_editor(provider_name=provider_name, settings=request_settings)

    try:
        edited_bytes, mime = await editor.edit_image(raw_images, prompt=prompt, options={})
    except NotImplementedError as e:
        raise HTTPException(status_code=501, detail=str(e))
    except Exception as e:
        logger.exception("Editing failed")
        raise HTTPException(status_code=500, detail=f"Editing failed: {e}")

    media_type = mime or images[0].content_type
    logger.info(f"Edit complete provider='{provider_name}' content_type='{media_type}' bytes={len(edited_bytes)}")
    return StreamingResponse(iter([edited_bytes]), media_type=media_type)
