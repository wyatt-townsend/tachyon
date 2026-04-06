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
  type SearchEvent = { event: "found"; data: { pathString: string } };

  onMount(() => {
    invoke("build_file_path_list").then((message) => (filesIndexed = true));
  });

  function cancelSearch() {
    invoke("cancel_search");
  }

  async function runSearch() {
    // End the previous search
    await cancelSearch();
    files = [];

    // Event handler
    const onEvent = new Channel<SearchEvent>();
    onEvent.onmessage = (message) => {
      if (message.event === "found") {
        files = [...files, message.data.pathString];
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
    <p class="page-alert" role="alert">Indexing Files</p>
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

  .search-dock {
    flex-shrink: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    padding-top: var(--space-sm);
  }
</style>
