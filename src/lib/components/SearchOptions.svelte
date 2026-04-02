<script lang="ts">
    let {
        drives,
        selectedDrive = $bindable(""),
        searchPattern = $bindable(""),
        disableOptions,
        onSubmit,
    } = $props();
</script>

<form onsubmit={onSubmit}>
    <div>
        <label for="searchPattern">Search Pattern</label>
        <input
            type="text"
            id="searchPattern"
            disabled={disableOptions}
            bind:value={searchPattern}
        />
    </div>
    <div>
        <select disabled={disableOptions} bind:value={selectedDrive}>
            {#each drives as drive (drive)}
                <option value={drive}>{drive}</option>
            {/each}
        </select>
    </div>
    <button type="submit">Search</button>
</form>
