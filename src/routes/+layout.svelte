<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<div class="app-shell">
  <div class="app-body">
    {@render children()}
  </div>
</div>

<style>
  .app-shell {
    min-height: 100dvh;
    display: flex;
    flex-direction: column;
    background: var(--color-bg);
    color: var(--color-text);
  }

  .app-body {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
</style>
