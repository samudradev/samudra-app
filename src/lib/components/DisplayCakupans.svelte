<script lang="ts">
    // APIs
    // Components
    // Stores
    // Types

    // Initialize values
    export let cakupans: string[] = [];
    // Event listeners
    // Callables
</script>

<div class="column">
    {#if cakupans.length != 0}
        konteks:
    {/if}
    {#each cakupans as cakupan}
        <div class="badge border-primary">{cakupan}</div>
    {/each}
</div>
