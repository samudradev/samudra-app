<script lang="ts">
    // APIs
    import { onMount } from "svelte";
    // Components
    import FormAppendCakupan from "./FormAppendCakupan.svelte";
    import FormAppendKataAsing from "./FormAppendKataAsing.svelte";
    import SelectGolonganKata from "./SelectGolonganKata.svelte";
    // Stores
    // Types
    import type { KonsepItem } from "../../bindings/KonsepItem";
    import type { KataAsingItem } from "../../bindings/KataAsingItem";

    // Initialize values
    export let konsep: KonsepItem;
    export let onSubmit: VoidFunction = () => {};
    export let index: number = 0;
    $: cakupan_list = [] as string[];
    $: kata_asing_list = [] as KataAsingItem[];
    let id_item: bigint;
    let keterangan_item: string;
    let golongan_kata_item: string;

    onMount(() => {
        if (konsep != null && konsep != undefined) {
            id_item = konsep.id;
            keterangan_item = konsep.keterangan;
            golongan_kata_item = konsep.golongan_kata;
            cakupan_list = konsep.cakupans;
            kata_asing_list = konsep.kata_asing;
        }
    });
    // Event listeners
    // Callables
    function save_new_konsep() {
        konsep = {
            id: id_item,
            keterangan: keterangan_item,
            golongan_kata: golongan_kata_item,
            cakupans: cakupan_list,
            kata_asing: kata_asing_list,
        };
        cakupan_list = [];
        kata_asing_list = [];
        keterangan_item = "";
    }
    console.log(cakupan_list);
</script>

<div class="indicator w-full mt-4">
    <div
        class="text-left card-bordered border-primary border-2 rounded-lg w-full"
    >
        <form
            on:submit|preventDefault={save_new_konsep}
            on:submit|preventDefault={onSubmit}
            class="space-y-4"
        >
            {index + 1}.
            <SelectGolonganKata bind:golongan_kata={golongan_kata_item} />
            <button type="submit" class="text-right indicator-item btn-primary"
                >+</button
            >
            <textarea
                class="textarea w-full"
                placeholder="konsep"
                bind:value={keterangan_item}
                required
            />
        </form>
        <FormAppendCakupan bind:cakupans={cakupan_list} />
        <FormAppendKataAsing bind:kata_asings={kata_asing_list} />
    </div>
</div>
