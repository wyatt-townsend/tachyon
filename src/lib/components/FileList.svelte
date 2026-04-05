<script lang="ts">
  let { files } = $props();
</script>

<section class="file-list" aria-label="Discovered paths">
  <div class="file-list-panel">
    <div class="file-list-header">
      <span class="file-list-title">Paths</span>
      {#if files.length > 0}
        <span class="file-list-meta">{files.length} files found</span>
      {/if}
    </div>
    <ul class="file-list-ul">
      {#each files as file (file)}
        <li class="file-list-li">{file}</li>
      {:else}
        <li class="file-list-li file-list-li-empty">
          No paths yet. Run a search from the options below.
        </li>
      {/each}
    </ul>
  </div>
</section>

<style>
  .file-list {
    width: min(92vw, 56rem);
    height: min(72vh, 16rem);
    max-width: 100%;
    flex: 1 1 auto;
    min-height: 12rem;
    display: flex;
    flex-direction: column;
  }

  .file-list-panel {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface-inset);
    box-shadow: inset 0 1px 0 rgb(255 255 255 / 0.03);
  }

  .file-list-header {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface);
    font-size: var(--text-xs);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-muted);
  }

  .file-list-title {
    font-weight: 600;
    color: var(--color-accent);
  }

  .file-list-meta {
    color: var(--color-text-muted);
  }

  .file-list-ul {
    list-style: none;
    margin: 0;
    padding: var(--space-xs) 0;
    overflow: auto;
    flex: 1;
    min-height: 0;
  }

  .file-list-li {
    margin: 0;
    padding: var(--space-xs) var(--space-md);
    border-left: 2px solid transparent;
    font-size: var(--text-sm);
    color: var(--color-text);
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast);
  }

  .file-list-li:not(.file-list-li-empty):hover {
    background: var(--color-row-hover);
    border-left-color: var(--color-accent-muted);
  }

  .file-list-li-empty {
    color: var(--color-text-muted);
    font-style: italic;
    border-left-color: transparent;
  }
</style>
