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
- GOOGLE_MODEL_ID=gemini-flash-image-edit (placeholder; use your actual Google model id)
- ALLOWED_ORIGINS=http://localhost:5173,http://127.0.0.1:5173

Run (with uv)
1) Install uv (https://github.com/astral-sh/uv). Then from `backend/`:
   uv sync
2) Start the API:
   uv run uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

Notes
- The Google provider call is a stub with placeholders for the correct endpoint and payload shape. Fill in the TODOs with the Google Generative AI Images Edit API you use (Vertex AI or Generative Language API) and model id.
- Qwen Edit and Kontext Pro are left as placeholders; implement their API calls in `app/services/qwen_editor.py` and `app/services/kontext_pro_editor.py`.

