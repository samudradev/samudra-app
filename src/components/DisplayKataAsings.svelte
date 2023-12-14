<script lang="ts">
    import type { KataAsingItem } from "../bindings/KataAsingItem";

    export let kata_asings: KataAsingItem[];
</script>

<div class="column">
    {#each kata_asings as kata_asing}
        <div class="badge border-secondary">
            {kata_asing.nama} ({kata_asing.bahasa})
        </div>
    {/each}
</div>
