<script lang="ts">
    import type { KataAsingItem } from "../bindings/KataAsingItem";

    export let kata_asings: KataAsingItem[];
</script>

<div class="column">
    {#if kata_asings.length != 0}
        bahasa lain:
    {/if}
    {#each kata_asings as kata_asing}
        <div class="badge border-secondary">
            <i class="pr-[2px]">{kata_asing.nama}</i><span
                >({kata_asing.bahasa})</span
            >
        </div>
    {/each}
</div>
