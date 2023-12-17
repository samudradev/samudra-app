<script lang="ts">
    // APIs
    // Components
    // Stores
    // Types
    import type { KataAsingItem } from "../../bindings/KataAsingItem";

    // Initialize values
    export let kata_asings: KataAsingItem[] = [];
    export let editable: boolean = false;
    export let label: boolean = false;
    // Event listeners
    // Callables
    function remove_item(index: number) {
        kata_asings.splice(index, 1);
        kata_asings = kata_asings;
    }
</script>

{#if kata_asings != null && kata_asings != undefined}
    <div class="column">
        {#if kata_asings.length != 0 || label}
            bahasa lain:
        {/if}
        {#each kata_asings as kata_asing, i}
            <div class="badge border-secondary">
                <i class="pr-[2px]">{kata_asing.nama}</i><span
                    >({kata_asing.bahasa})</span
                >
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
