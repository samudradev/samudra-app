<script lang="ts">
    // APIs
    // Components
    // Stores
    // Types

    // Initialize values
    export let cakupans: string[] = [];
    export let editable: boolean = false;
    export let label: boolean = false;
    // Event listeners
    // Callables
    function remove_item(index: number) {
        cakupans.splice(index, 1);
        cakupans = cakupans;
    }
</script>

{#if cakupans != null && cakupans != undefined}
    <div class="column">
        {#if cakupans.length != 0 || label}
            konteks:
        {/if}
        {#each cakupans as cakupan, i}
            <div class="badge border-primary">
                {cakupan}
                {#if editable}
                    <button
                        class="badge p-0 ml-2 text-center border-primary w-6"
                        on:click={() => {
                            remove_item(i);
                        }}
                    >
                        x
                    </button>
                {/if}
            </div>
        {/each}
    </div>
{/if}
