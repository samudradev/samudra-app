<script lang="ts">
    export let cakupans: string[];
</script>

<div class="column">
    {#if cakupans.length != 0}
        konteks:
    {/if}
    {#each cakupans as cakupan}
        <div class="badge border-primary">{cakupan}</div>
    {/each}
</div>
