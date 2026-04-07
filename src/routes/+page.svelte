<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import FileList from "$lib/components/FileList.svelte";
  import SearchOptions from "$lib/components/SearchOptions.svelte";

  let selectedDrive: string = $state<string>("");
  let searchPattern: string = $state<string>("");
  let files: string[] = $state<string[]>([]);
  let findError: string | null = $state<string | null>(null);
  let filesIndexed: Boolean = $state(false);
  let indexedCount: number = $state(0);
  type SearchEvent = { event: "found"; data: { pathString: string } };
  type IndexEvent =
    | { event: "progress"; data: { count: number } }
    | { event: "done"; data: { total: number } };

  let searchId = 0;

  onMount(async () => {
    const onEvent = new Channel<IndexEvent>();
    onEvent.onmessage = (message) => {
      if (message.event === "progress") {
        indexedCount = message.data.count;
      } else if (message.event === "done") {
        indexedCount = message.data.total;
        filesIndexed = true;
      }
    };
    await invoke("build_file_path_list", { onEvent });
    filesIndexed = true;
  });

  function cancelSearch() {
    invoke("cancel_search");
  }

  async function runSearch() {
    // End the previous search
    await cancelSearch();
    files = [];
    const myId = ++searchId;

    if (searchPattern.trim().length <= 0) return;

    // Event handler
    const onEvent = new Channel<SearchEvent>();
    onEvent.onmessage = (message) => {
      if (searchId !== myId) return;
      if (message.event === "found") {
        files.push(message.data.pathString);
      }
    };

    // Start the search
    try {
      await invoke("fuzzy_filter", {
        pattern: searchPattern,
        onEvent,
      });

      findError = null;
    } catch (err) {
      findError = err instanceof Error ? err.message : String(err);
    }
  }
</script>

<main class="page-workspace">
  {#if findError}
    <p class="page-alert" role="alert">{findError}</p>
  {/if}

  {#if filesIndexed}
    <div class="list-stage">
      <FileList {files} />
    </div>

    <div class="search-dock">
      <SearchOptions
        bind:selectedDrive
        bind:searchPattern
        onSearch={runSearch}
      />
    </div>
  {:else}
    <div class="indexing-status" role="status">
      <span class="indexing-spinner"></span>
      <span class="indexing-label">
        Indexing files{#if indexedCount > 0}
          &mdash; {indexedCount.toLocaleString()} found{/if}&hellip;
      </span>
    </div>
  {/if}
</main>

<style>
  .page-workspace {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    width: 100%;
    box-sizing: border-box;
    padding: var(--space-md) var(--space-sm);
    gap: var(--space-md);
  }

  .page-alert {
    flex-shrink: 0;
    width: 100%;
    margin: 0;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-danger);
    border-radius: var(--radius-md);
    background: var(--color-surface-inset);
    color: var(--color-danger);
    font-size: var(--text-sm);
    transition:
      border-color var(--transition-fast),
      background var(--transition-fast);
  }

  .list-stage {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    width: 100%;
  }

  .indexing-status {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface-inset);
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }

  .indexing-spinner {
    flex-shrink: 0;
    width: 14px;
    height: 14px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .indexing-label {
    white-space: nowrap;
  }

  .search-dock {
    flex-shrink: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding-top: var(--space-sm);
  }
</style>
