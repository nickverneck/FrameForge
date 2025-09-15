<script>
  import Upload from './lib/Upload.svelte'
</script>

<main>
  <h1>FrameForge</h1>
  <p class="subtitle">Stage your room photo with AI</p>
  <Upload />
</main>

<style>
  :global(body){
    margin:0; font-family: system-ui, -apple-system, Segoe UI, Roboto, Ubuntu, Cantarell, Noto Sans, Arial, sans-serif; background:#0b0b0b; color:#eaeaea;
  }
  main { max-width: 860px; margin: 24px auto; padding: 16px; }
  h1 { font-size: 28px; margin: 8px 0; }
  .subtitle { color: #b5b5b5; margin-bottom: 16px; }
</style>

