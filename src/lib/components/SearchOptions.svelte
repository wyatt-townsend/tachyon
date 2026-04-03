<script lang="ts">
  let {
    drives,
    selectedDrive = $bindable(""),
    searchPattern = $bindable(""),
    disableOptions,
    onSubmit,
  } = $props();

  // Select the first item by default
  $effect(() => {
    if (drives.length === 0) return;
    if (selectedDrive === "" || !drives.includes(selectedDrive)) {
      selectedDrive = drives[0];
    }
  });
</script>

<form class="search-form" onsubmit={onSubmit}>
  <div class="search-field">
    <label class="search-label" for="searchPattern">Search pattern</label>
    <input
      class="search-input"
      type="text"
      id="searchPattern"
      disabled={disableOptions}
      bind:value={searchPattern}
    />
  </div>
  <div class="search-field">
    <label class="search-label" for="searchDrive">Drive</label>
    <select
      class="search-select"
      id="searchDrive"
      disabled={disableOptions || drives.length <= 1}
      bind:value={selectedDrive}
    >
      {#each drives as drive (drive)}
        <option value={drive}>{drive}</option>
      {/each}
    </select>
  </div>
  <button class="search-button" type="submit" disabled={disableOptions}>
    Search
  </button>
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

  .search-input,
  .search-select {
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface-inset);
    color: var(--color-text);
    min-height: 2rem;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast);
  }

  .search-input:hover:not(:disabled),
  .search-select:hover:not(:disabled) {
    border-color: var(--color-accent-muted);
  }

  .search-input:focus-visible,
  .search-select:focus-visible {
    outline: none;
    border-color: var(--color-border-focus);
    box-shadow: 0 0 0 1px var(--color-border-focus);
  }

  .search-input:disabled,
  .search-select:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .search-select {
    min-width: 6rem;
    cursor: pointer;
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
