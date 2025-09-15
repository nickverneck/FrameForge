<script>
  import { editImage } from './api.js'

  let file = null;
  let fileInput; // ref for the hidden input bound via bind:this
  let previewUrl = '';
  let resultUrl = '';
  let prompt = 'Stage this room with minimalist modern furniture in neutral tones.';
  let provider = 'google';
  let loading = false;
  let error = '';

  function onFile(e) {
    const f = e?.target?.files?.[0] || e?.dataTransfer?.files?.[0];
    if (!f) return;
    file = f;
    resultUrl = '';
    error = '';
    const reader = new FileReader();
    reader.onload = () => { previewUrl = reader.result; };
    reader.readAsDataURL(f);
  }

  async function onSubmit() {
    if (!file) { error = 'Please choose an image first.'; return; }
    loading = true; error = '';
    try {
      const blob = await editImage({ file, prompt, provider });
      resultUrl = URL.createObjectURL(blob);
    } catch (e) {
      error = e?.message || 'Failed to edit image';
    } finally {
      loading = false;
    }
  }
</script>

<div class="card">
  <div
    class="drop"
    on:dragover|preventDefault
    on:drop|preventDefault={onFile}
    on:click={() => fileInput.click()}
  >
    <input bind:this={fileInput} type="file" accept="image/*" on:change={onFile} hidden />
    {#if previewUrl}
      <img alt="preview" src={previewUrl} />
    {:else}
      <p>Drag & drop an image here or click to select</p>
    {/if}
  </div>

  <div class="controls">
    <label>
      Prompt
      <textarea bind:value={prompt} rows="3" placeholder="Describe how to stage the room..."></textarea>
    </label>
    <label>
      Provider
      <select bind:value={provider}>
        <optgroup label="Google">
          <option value="google">Nano-banana (Google)</option>
        </optgroup>
        <optgroup label="FAL.ai">
          <option value="fal:fal-ai/nano-banana/edit">Nano-banana (FAL)</option>
          <option value="fal:fal-ai/qwen-image-edit">Qwen Image Edit (FAL)</option>
          <option value="fal:fal-ai/bytedance/seedream/v4/edit">Seedream v4 Edit (FAL)</option>
          <option value="fal:fal-ai/flux-kontext/dev">Flux Kontext Dev (FAL)</option>
        </optgroup>
      </select>
    </label>
    <button class="primary" on:click={onSubmit} disabled={loading}>
      {#if loading}Editing...{/if}
      {#if !loading}Edit Image{/if}
    </button>
    {#if error}
      <div class="error">{error}</div>
    {/if}
  </div>

  {#if resultUrl}
    <div class="result">
      <h3>Edited Result</h3>
      <img alt="result" src={resultUrl} />
      <a class="download" href={resultUrl} download="edited.png">Download</a>
    </div>
  {/if}
</div>

<style>
  .card { background:#161616; border:1px solid #2a2a2a; border-radius: 12px; padding: 16px; }
  .drop { border: 2px dashed #3a3a3a; border-radius: 10px; padding: 16px; min-height: 180px; display:flex; align-items:center; justify-content:center; text-align:center; cursor: pointer; }
  .drop p { color:#9a9a9a; }
  .drop img { max-width: 100%; max-height: 420px; border-radius: 8px; }
  .controls { display:grid; gap:12px; margin-top:14px; }
  label { display:grid; gap:6px; font-size:14px; }
  textarea, select { width:100%; padding:10px; border-radius:8px; border:1px solid #333; background:#0f0f0f; color:#eaeaea; }
  button.primary { padding: 10px 14px; background:#4c8bf5; color:white; border:none; border-radius:8px; cursor:pointer; width:fit-content; }
  button.primary[disabled] { opacity: .6; cursor: not-allowed; }
  .error { color: #ff6b6b; }
  .result { margin-top:16px; }
  .result img { max-width: 100%; border-radius:8px; border:1px solid #2a2a2a; }
  .download { display:inline-block; margin-top:8px; color:#eaeaea; text-decoration: underline; }
  @media (max-width: 640px) { .drop { min-height: 140px; } }
</style>
