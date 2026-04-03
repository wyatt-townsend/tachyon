<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<div class="app-shell">
  <header class="app-header">
    <h1 class="app-title">tachyon</h1>
  </header>
  <div class="app-body">
    {@render children()}
  </div>
  <footer class="app-footer">
    <p class="app-footer-text">© 2026 tachyon</p>
  </footer>
</div>

<style>
  .app-shell {
    min-height: 100dvh;
    display: flex;
    flex-direction: column;
    background: var(--color-bg);
    color: var(--color-text);
  }

  .app-header {
    flex-shrink: 0;
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface-inset);
  }

  .app-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: lowercase;
    color: var(--color-text);
  }

  .app-body {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .app-footer {
    flex-shrink: 0;
    padding: var(--space-xs) var(--space-lg);
    border-top: 1px solid var(--color-border);
    background: var(--color-surface-inset);
  }

  .app-footer-text {
    margin: 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
