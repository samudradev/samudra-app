<script lang="ts">
    import type { KonsepItem } from "../bindings/KonsepItem";
    import type { LemmaItem } from "../bindings/LemmaItem";

    import FormAppendCakupan from "./FormAppendCakupan.svelte";
    import FormAppendKataAsing from "./FormAppendKataAsing.svelte";
    import SelectGolonganKata from "./SelectGolonganKata.svelte";

    export let data: LemmaItem;
    let keterangan_item: string;
    let golongan_kata_item: string;

    $: cakupan_list = [];
    $: kata_asing_list = [];

    function append_new_konsep() {
        data.konseps.push(
            Object.assign({} as KonsepItem, {
                id: null,
                keterangan: keterangan_item,
                golongan_kata: golongan_kata_item,
                cakupans: cakupan_list,
                kata_asing: kata_asing_list,
            }),
        );
        data = data; // To force reload
        cakupan_list = [];
        kata_asing_list = [];
        keterangan_item = "";
    }
</script>

<div class="indicator w-full mt-4">
    <div
        class="text-left card-bordered p-2 border-primary border-2 rounded-lg w-full"
    >
        <form on:submit|preventDefault={append_new_konsep} class="space-y-4">
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
