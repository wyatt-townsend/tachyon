<script lang="ts">
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import FileList from "$lib/components/FileList.svelte";
  import SearchOptions from "$lib/components/SearchOptions.svelte";

  type SearchEvent =
    | { event: "progress"; data: { pathString: string } }
    | { event: "result"; data: { total: number } };

  let drives: string[] = $state<string[]>([]);
  let selectedDrive: string = $state<string>("");
  let searchPattern: string = $state<string>("");
  let files: string[] = $state<string[]>([]);
  let searching = $state<boolean>(false);
  let walkError: string | null = $state<string | null>(null);

  onMount(async () => {
    invoke("get_drives").then((message) => (drives = message as string[]));
  });

  async function startWalk(e: SubmitEvent) {
    e.preventDefault();
    walkError = null;
    files = [];
    searching = true;

    const onEvent = new Channel<SearchEvent>();
    onEvent.onmessage = (message: SearchEvent) => {
      if (message.event === "progress") {
        files = [...files, message.data.pathString];
      } else {
        searching = false;
      }
    };

    try {
      await invoke("walk_directory", {
        root: selectedDrive,
        pattern: searchPattern,
        onEvent,
      });
    } catch (err) {
      walkError = err instanceof Error ? err.message : String(err);
    } finally {
      searching = false;
    }
  }
</script>

<main class="page-workspace">
  {#if walkError}
    <p class="page-alert" role="alert">{walkError}</p>
  {/if}

  <div class="list-stage">
    <FileList {files} {searching} />
  </div>

  <div class="search-dock">
    <SearchOptions
      {drives}
      bind:selectedDrive
      bind:searchPattern
      disableOptions={searching}
      onSubmit={startWalk}
    />
  </div>
</main>

<style>
  .page-workspace {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    padding: var(--space-md) var(--space-lg);
    gap: var(--space-md);
  }

  .page-alert {
    flex-shrink: 0;
    margin: 0;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-danger);
    border-radius: var(--radius-sm);
    background: var(--color-surface-inset);
    color: var(--color-danger);
    font-size: var(--text-sm);
    transition: border-color var(--transition-fast), background var(--transition-fast);
  }

  .list-stage {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .search-dock {
    flex-shrink: 0;
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    padding-top: var(--space-sm);
  }
</style>
