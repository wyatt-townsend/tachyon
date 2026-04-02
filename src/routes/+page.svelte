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

<main class="container">
  <SearchOptions
    {drives}
    bind:selectedDrive
    bind:searchPattern
    disableOptions={searching}
    onSubmit={startWalk}
  />

  {#if walkError}
    <p role="alert">{walkError}</p>
  {/if}

  <FileList {files} {searching} />
</main>
