# FrameForge

Room staging app: upload an unfurnished room photo, get back an edited image staged with furniture using pluggable providers. Backend is FastAPI (Python), frontend is Svelte (Vite). Default provider is Google "nano-banana" (Gemini Flash image editing). Qwen Edit and Kontext Pro are stubbed for future use.

Directories
- `backend/`: FastAPI service with provider abstraction.
- `frontend/`: Svelte UI with drag-and-drop upload.

Backend – Run (uv)
1) Copy env and edit as needed:
   cp backend/.env.example backend/.env
2) Install deps with uv:
   cd backend && uv sync
3) Start API:
   uv run uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

Frontend – Run (Vite)
1) Install deps:
   cd frontend && npm install
2) Point UI to API (optional):
   echo "VITE_API_BASE=http://localhost:8000" > .env
3) Start dev server:
   npm run dev

Provider Notes
- Google (nano-banana): `backend/app/services/google_nano_banana.py` contains a stub. Replace the endpoint and payload with the official Google Images Edit API and set `GOOGLE_API_KEY` + `GOOGLE_MODEL_ID` in `backend/.env`.
- Qwen Edit: `backend/app/services/qwen_editor.py` – not implemented yet.
- Kontext Pro: `backend/app/services/kontext_pro_editor.py` – not implemented yet.

API
- POST `/api/edit` (multipart): fields `image` (file), `prompt` (string, optional), `provider` (string: `google|qwen|kontext`, optional; defaults to `google`). Returns edited image bytes.
- GET `/providers`: discover providers.
- GET `/health`: health check.

