<script lang="ts">
  import { revealItemInDir } from "@tauri-apps/plugin-opener";

  let { files } = $props();

  async function openContainingFolder(path: string) {
    await revealItemInDir(path);
  }
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
        <li class="file-list-li">
          <button
            class="file-list-btn"
            onclick={() => openContainingFolder(file)}
            title="Open containing folder"
          >{file}</button>
        </li>
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
    width: 100%;
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .file-list-panel {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border: 1px solid color-mix(in srgb, var(--color-border) 85%, transparent);
    border-radius: var(--radius-md);
    background: var(--surface-acrylic);
    box-shadow: var(--shadow-subtle);
    backdrop-filter: var(--acrylic-filter);
    -webkit-backdrop-filter: var(--acrylic-filter);
  }

  .file-list-header {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid
      color-mix(in srgb, var(--color-border) 45%, transparent);
    background: transparent;
    font-size: var(--text-xs);
    font-weight: 600;
    letter-spacing: 0.01em;
    color: var(--color-text-muted);
  }

  .file-list-title {
    font-weight: 600;
    color: var(--color-accent);
  }

  .file-list-meta {
    font-weight: 500;
    color: var(--color-text-muted);
  }

  .file-list-ul {
    list-style: none;
    margin: 0;
    padding: var(--space-xs) 0;
    overflow: auto;
    flex: 1;
    min-height: 0;
    scrollbar-width: thin;
    scrollbar-color: color-mix(
        in srgb,
        var(--color-text-muted) 45%,
        transparent
      )
      transparent;
  }

  .file-list-ul::-webkit-scrollbar {
    width: 10px;
    height: 10px;
  }

  .file-list-ul::-webkit-scrollbar-track {
    background: transparent;
  }

  .file-list-ul::-webkit-scrollbar-thumb {
    background: color-mix(in srgb, var(--color-text-muted) 40%, transparent);
    border-radius: 5px;
    border: 2px solid transparent;
    background-clip: padding-box;
  }

  .file-list-ul::-webkit-scrollbar-thumb:hover {
    background: color-mix(in srgb, var(--color-text-muted) 58%, transparent);
    background-clip: padding-box;
  }

  .file-list-li {
    margin: 0;
    padding: var(--space-xs) var(--space-md);
    border-left: 2px solid transparent;
    border-radius: var(--radius-sm);
    margin-inline: var(--space-xs);
    font-size: var(--text-sm);
    color: var(--color-text);
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast);
  }

  .file-list-li:not(.file-list-li-empty):hover {
    background: color-mix(in srgb, var(--color-text) 6%, transparent);
    border-left-color: color-mix(in srgb, var(--color-accent) 55%, transparent);
  }

  .file-list-btn {
    display: block;
    width: 100%;
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    font: inherit;
    color: inherit;
    text-align: left;
    cursor: pointer;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-list-li:not(.file-list-li-empty):active {
    background: color-mix(in srgb, var(--color-accent) 12%, transparent);
    border-left-color: var(--color-accent);
  }

  .file-list-li-empty {
    color: var(--color-text-muted);
    font-style: italic;
    border-left-color: transparent;
  }
</style>
