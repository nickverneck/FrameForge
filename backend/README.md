FrameForge Backend (FastAPI)

Overview
- FastAPI service that accepts an unfurnished room photo and a prompt, then delegates to an image-editing provider.
- Default provider is Google "nano-banana" (Gemini Flash image edit). Stubs are included for Qwen Edit and Kontext Pro.

Endpoints
- POST /api/edit: multipart upload: `image` (file), `prompt` (string, optional), `provider` (string, optional: `google`, `qwen`, `kontext`). Returns edited image bytes.
- GET /health: returns simple ok for health checks.
- GET /providers: lists available providers.

Env Vars (.env)
- GOOGLE_API_KEY=your_key_here
- GEMINI_API_KEY=your_key_here (either works)
- GOOGLE_MODEL_ID=gemini-2.5-flash-image-preview (or your preferred model)
- ALLOWED_ORIGINS=["http://localhost:5173","http://127.0.0.1:5173"]  <- JSON array

Run (with uv)
1) Install uv (https://github.com/astral-sh/uv). Then from `backend/`:
   uv sync
2) Start the API:
   uv run uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

Notes
- The Google provider now uses the `google-genai` SDK with streaming to return inline image data. Set `GOOGLE_API_KEY` or `GEMINI_API_KEY` and an image-capable model id.
- `ALLOWED_ORIGINS` must be valid JSON if set in `.env` because `pydantic-settings` parses list fields from JSON.
- FAL support: Set `FAL_KEY` and use provider values like `fal:fal-ai/nano-banana/edit`, `fal:fal-ai/qwen-image-edit`, `fal:fal-ai/bytedance/seedream/v4/edit`, `fal:fal-ai/flux-kontext/dev`. See `app/services/fal_editor.py`.
