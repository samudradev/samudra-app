<script lang="ts">
    // APIs
    // Components
    import DisplayCakupans from "./DisplayCakupans.svelte";
    import DisplayKataAsings from "./DisplayKataAsings.svelte";
    // Stores
    // Types
    import type { KonsepItem } from "../../bindings/KonsepItem";
    import FormAppendKonsep from "./FormAppendKonsep.svelte";

    // Initialize values
    export let konseps: KonsepItem[] = [];
    export let editable: boolean = false;
    $: edit_at = -1 as Number;
    // Event listeners
    // Callables
    function set_edit_at(index: number) {
        console.log(konseps[index]);
        edit_at = index;
    }

    function update_value_at(konsep: KonsepItem, index: number) {
        console.log(konsep);
        console.log(index);
        konseps.splice(index, 1, konsep);
        konseps = konseps;
        edit_at = -1;
    }
</script>

{#each konseps as konsep, i}
    {#if i == edit_at}
        <FormAppendKonsep
            bind:konsep
            index={i}
            onSubmit={() => {
                update_value_at(konsep, i);
            }}
        />
    {:else}
        <div class="text-left">
            {i + 1}.
            <span class="badge bg-neutral-300 border-0"
                >{konsep.golongan_kata}</span
            >
            {konsep.keterangan}
            {#if editable}
                <button
                    on:click={() => {
                        set_edit_at(i);
                    }}>Edit</button
                >
            {/if}
            <DisplayCakupans bind:cakupans={konsep.cakupans} />
            <DisplayKataAsings bind:kata_asings={konsep.kata_asing} />
        </div>
    {/if}
{/each}
