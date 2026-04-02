<script lang="ts">
    let { files, searching } = $props();
</script>

<section aria-label="Discovered paths">
    <div>
        <span>Paths</span>
        {#if searching}
            <span>...</span>
        {:else if files.length > 0}
            <span>{files.length} files found</span>
        {/if}
    </div>
    <ul class="path-list">
        {#each files as file (file)}
            <li>
                {file}
            </li>
        {:else}
            <li class="path-item empty">No paths yet. Run a search above.</li>
        {/each}
    </ul>
</section>
