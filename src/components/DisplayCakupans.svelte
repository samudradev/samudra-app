<script lang="ts">
    export let cakupans: string[];
</script>

<div class="column">
    {#each cakupans as cakupan}
        <div class="badge border-primary">{cakupan}</div>
    {/each}
</div>
