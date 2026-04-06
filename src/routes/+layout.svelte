<script lang="ts">
  import { onMount } from "svelte";
  import { isTauri } from "@tauri-apps/api/core";
  import "../app.css";

  let { children } = $props();

  onMount(() => {
    if (isTauri()) {
      document.documentElement.dataset.tauri = "";
    }
  });
</script>

<div class="app-shell">
  <div class="app-shell-backdrop" aria-hidden="true"></div>
  <div class="app-body">
    {@render children()}
  </div>
</div>

<style>
  .app-shell {
    position: relative;
    isolation: isolate;
    height: 100dvh;
    min-height: 0;
    max-height: 100dvh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: var(--color-bg);
    color: var(--color-text);
  }

  :global(html[data-tauri]) .app-shell {
    background: transparent;
  }

  .app-shell-backdrop {
    position: fixed;
    inset: 0;
    z-index: -1;
    pointer-events: none;
    background:
      radial-gradient(
        ellipse 120% 85% at 50% 0%,
        color-mix(in srgb, var(--color-accent) 22%, transparent),
        transparent 58%
      ),
      radial-gradient(
        ellipse 90% 70% at 100% 100%,
        color-mix(in srgb, var(--color-surface-inset) 45%, transparent),
        transparent 52%
      );
    opacity: 0.55;
  }

  :global(html[data-tauri]) .app-shell-backdrop {
    opacity: 0.22;
  }

  .app-body {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
</style>
