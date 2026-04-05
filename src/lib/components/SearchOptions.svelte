<script lang="ts">
  let {
    selectedDrive = $bindable(""),
    searchPattern = $bindable(""),
    onSearch,
  } = $props();
</script>

<form class="search-form" onsubmit={onSearch}>
  <div class="search-field">
    <label class="search-label" for="searchPattern">Search pattern</label>
    <input
      class="search-input"
      type="text"
      id="searchPattern"
      bind:value={searchPattern}
    />
  </div>
  <button class="search-button" type="submit"> Search </button>
</form>

<style>
  .search-form {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: flex-end;
    gap: var(--space-md);
    width: auto;
    max-width: 100%;
  }

  .search-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    min-width: 0;
  }

  .search-field:first-child {
    flex: 1 1 12rem;
    max-width: 20rem;
  }

  .search-label {
    font-size: var(--text-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
  }

  .search-button {
    padding: var(--space-sm) var(--space-lg);
    border: 1px solid var(--color-accent-muted);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    color: var(--color-text);
    cursor: pointer;
    min-height: 2rem;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast),
      transform var(--transition-active);
  }

  .search-button:hover:not(:disabled) {
    background: var(--color-row-hover);
    border-color: var(--color-accent);
    color: var(--color-text);
  }

  .search-button:active:not(:disabled) {
    transform: scale(0.98);
    background: var(--color-surface-inset);
  }

  .search-button:focus-visible {
    outline: none;
    box-shadow: 0 0 0 1px var(--color-border-focus);
  }

  .search-button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
</style>
