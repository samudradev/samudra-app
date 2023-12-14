<script lang="ts">
    import type { KonsepItem } from "../bindings/KonsepItem";
    import DisplayCakupans from "./DisplayCakupans.svelte";
    import DisplayKataAsings from "./DisplayKataAsings.svelte";

    export let konseps: KonsepItem[];
</script>

{#each konseps as konsep, i}
    <div class="text-left">
        {i + 1}.
        <span class="badge bg-neutral-300 border-0">{konsep.golongan_kata}</span
        >
        {konsep.keterangan}
        <DisplayCakupans bind:cakupans={konsep.cakupans} />
        <DisplayKataAsings bind:kata_asings={konsep.kata_asing} />
    </div>
{/each}
