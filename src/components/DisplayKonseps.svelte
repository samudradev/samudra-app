<script lang="ts">
    import type { KonsepItem } from "../bindings/KonsepItem";

    export let konseps: KonsepItem[];
</script>

{#each konseps as konsep, i}
    <div class="text-left">
        {i + 1}.
        <span class="badge">{konsep.golongan_kata}</span>
        {konsep.keterangan}
        {#if konsep.cakupans != null}
            <div class="column">
                {#each konsep.cakupans as cakupan}
                    <div class="badge">{cakupan}</div>
                {/each}
            </div>
        {/if}
        {#if konsep.kata_asing != null}
            <div class="column">
                {#each konsep.kata_asing as kata_asing}
                    <div class="badge">
                        {kata_asing.nama} ({kata_asing.bahasa})
                    </div>
                {/each}
            </div>
        {/if}
    </div>
{/each}
