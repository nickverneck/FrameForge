//export const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:8000';

export async function editImage({ file, prompt, provider }) {
  const fd = new FormData();
  fd.append('image', file);
  if (prompt) fd.append('prompt', prompt);
  if (provider) fd.append('provider', provider);

  const res = await fetch(`/api/edit`, {
    method: 'POST',
    body: fd
  });
  if (!res.ok) {
    let detail = 'Failed to edit image';
    try { const data = await res.json(); detail = data?.detail || detail; } catch {}
    throw new Error(detail);
  }
  const blob = await res.blob();
  return blob;
}

