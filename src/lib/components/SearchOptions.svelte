<script lang="ts">
  let {
    selectedDrive = $bindable(""),
    searchPattern = $bindable(""),
    onSearch,
  } = $props();

  const DEBOUNCE_MS = 1000;
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  function scheduleSearch() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      onSearch();
    }, DEBOUNCE_MS);
  }
</script>

<form class="search-form">
  <div class="search-field">
    <label class="search-label" for="searchPattern">Search pattern</label>
    <input
      class="search-input"
      type="text"
      id="searchPattern"
      bind:value={searchPattern}
      oninput={scheduleSearch}
    />
  </div>
</form>

<style>
  .search-form {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: center;
    gap: var(--space-md);
    width: 100%;
    max-width: 22rem;
  }

  .search-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    min-width: 0;
    flex: 1 1 12rem;
    max-width: 22rem;
  }

  .search-label {
    font-size: var(--text-xs);
    font-weight: 600;
    letter-spacing: 0.01em;
    color: var(--color-text-muted);
  }

  .search-input {
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid color-mix(in srgb, var(--color-border) 85%, transparent);
    border-radius: var(--radius-md);
    background: var(--surface-acrylic);
    color: var(--color-text);
    box-shadow: var(--shadow-subtle);
    backdrop-filter: var(--acrylic-filter);
    -webkit-backdrop-filter: var(--acrylic-filter);
    outline: none;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast);
  }

  .search-input:hover {
    border-color: color-mix(in srgb, var(--color-border-focus) 55%, var(--color-border));
  }

  .search-input:focus-visible {
    border-color: var(--color-border-focus);
    box-shadow:
      var(--shadow-subtle),
      0 0 0 2px color-mix(in srgb, var(--color-border-focus) 35%, transparent);
  }

  .search-input::placeholder {
    color: var(--color-text-muted);
  }
</style>
