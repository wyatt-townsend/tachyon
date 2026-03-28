<script lang="ts">
  import { invoke, Channel } from "@tauri-apps/api/core";

  type SearchEvent =
    | { event: "progress"; data: { pathString: string } }
    | { event: "result"; data: { total: number } };

  let rows: string[] = [];
  let pathRoot = "";
  let searchPattern = "";
  let totalReported = 0;
  let walking = false;
  let walkError: string | null = null;

  const onEvent = new Channel<SearchEvent>();
  onEvent.onmessage = (message) => {
    if (message.event === "progress") {
      rows = [...rows, message.data.pathString];
    } else {
      totalReported = message.data.total;
    }
  };

  async function startWalk(e: SubmitEvent) {
    e.preventDefault();
    walkError = null;
    rows = [];
    totalReported = 0;
    walking = true;
    try {
      await invoke("walk_directory", {
        root: pathRoot,
        pattern: searchPattern,
        onEvent,
      });
    } catch (err) {
      walkError = err instanceof Error ? err.message : String(err);
    } finally {
      walking = false;
    }
  }
</script>

<main class="container">
  <h1>Tachyon</h1>

  <form class="search-form" onsubmit={startWalk}>
    <p class="search-label">Search for files</p>
    <div class="row">
      <input
        id="path-root-input"
        placeholder="Enter a path..."
        bind:value={pathRoot}
        disabled={walking}
      />
      <div class="row">
        <input
          id="path-root-input"
          placeholder="Enter a pattern..."
          bind:value={searchPattern}
          disabled={walking}
        />
        <button type="submit" disabled={walking || !pathRoot.trim()}>
          {walking ? "Searching…" : "Search"}
        </button>
      </div>
    </div>
  </form>

  {#if walkError}
    <p class="error" role="alert">{walkError}</p>
  {/if}

  <section class="list-section" aria-label="Discovered paths">
    <div class="list-header">
      <span>Paths</span>
      {#if totalReported > 0 && !walking}
        <span class="list-meta">{totalReported} entries</span>
      {:else if rows.length > 0}
        <span class="list-meta">{rows.length} shown…</span>
      {/if}
    </div>
    <ul class="path-list">
      {#each rows as row (row)}
        <li class="path-item">
          {row}
        </li>
      {:else}
        <li class="path-item empty">No paths yet. Run a search above.</li>
      {/each}
    </ul>
  </section>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0 auto;
    padding: 2rem 1.5rem 2rem;
    max-width: 52rem;
    min-height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 1.25rem;
    text-align: center;
  }

  .search-form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
  }

  .search-label {
    margin: 0;
  }

  .row {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    max-width: 40rem;
  }

  .error {
    margin: 0;
    color: #b42318;
    font-size: 0.9rem;
  }

  .list-section {
    text-align: left;
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    border: 1px solid rgba(15, 15, 15, 0.12);
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.6);
    overflow: hidden;
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.6rem 0.85rem;
    font-size: 0.85rem;
    font-weight: 600;
    border-bottom: 1px solid rgba(15, 15, 15, 0.1);
    background: rgba(255, 255, 255, 0.9);
  }

  .list-meta {
    font-weight: 500;
    color: #5c5c5c;
    font-size: 0.8rem;
  }

  .path-list {
    list-style: none;
    margin: 0;
    padding: 0.35rem 0;
    max-height: min(55vh, 28rem);
    overflow-y: auto;
    font-family: ui-monospace, "Cascadia Code", "Segoe UI Mono", monospace;
    font-size: 0.8rem;
    line-height: 1.45;
  }

  .path-item {
    padding: 0.35rem 0.85rem;
    word-break: break-all;
    border-bottom: 1px solid transparent;
  }

  .path-item:nth-child(even) {
    background: rgba(0, 0, 0, 0.03);
  }

  .path-item.empty {
    color: #737373;
    font-style: italic;
    font-family: inherit;
    text-align: center;
    padding: 1.5rem 1rem;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #path-root-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    .list-section {
      border-color: rgba(255, 255, 255, 0.12);
      background: rgba(0, 0, 0, 0.25);
    }

    .list-header {
      border-bottom-color: rgba(255, 255, 255, 0.1);
      background: rgba(0, 0, 0, 0.35);
    }

    .list-meta {
      color: #b0b0b0;
    }

    .path-item:nth-child(even) {
      background: rgba(255, 255, 255, 0.04);
    }

    .path-item.dir {
      color: #7eb6ff;
    }

    .path-item.empty {
      color: #9a9a9a;
    }

    .error {
      color: #fca5a5;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
