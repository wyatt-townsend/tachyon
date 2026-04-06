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
</style>
