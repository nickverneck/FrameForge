FrameForge Frontend (Svelte + Vite)

Overview
- Simple drag-and-drop or click-to-upload UI. Sends the image and prompt to the FastAPI backend.

Dev Setup
1) From `frontend/`: install deps
   npm install
2) Start dev server
   npm run dev

Config
- Backend URL: set `VITE_API_BASE` in `.env` (e.g., `VITE_API_BASE=http://localhost:8000`). Defaults to `http://localhost:8000`.

