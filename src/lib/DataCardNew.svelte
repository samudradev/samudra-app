<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import _ from "lodash";
    import type { KonsepItem } from "../bindings/KonsepItem";
    import { invoke } from "@tauri-apps/api";
    import SelectGolonganKata from "../components/SelectGolonganKata.svelte";
    import FormAppendCakupan from "../components/FormAppendCakupan.svelte";
    import FormAppendKataAsing from "../components/FormAppendKataAsing.svelte";
    import DisplayKonseps from "../components/DisplayKonseps.svelte";

    $: data = {
        id: null,
        lemma: "",
        konseps: [],
    } as LemmaItem;

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

    async function insert_lemma() {
        append_new_konsep();
        let a = await invoke("insert_lemma", { item: data });
        data = {
            id: null,
            lemma: "",
            konseps: [],
        } as LemmaItem;
    }
</script>

<div class="card card-normal m-2 w-[40em] bg-blue-100 shadow-xl">
    <div class="card-body">
        <div class="join">
            <input
                type="text"
                bind:value={data.lemma}
                class="input input-bordered w-full join-item"
            />
            <button class="btn-primary join-item" on:click={insert_lemma}>
                Save
            </button>
        </div>
        <div class="indicator w-full mt-4">
            <div
                class="text-left card-bordered p-2 border-primary border-2 rounded-lg w-full"
            >
                <form
                    on:submit|preventDefault={append_new_konsep}
                    class="space-y-4"
                >
                    <SelectGolonganKata
                        bind:golongan_kata={golongan_kata_item}
                    />
                    <button
                        type="submit"
                        class="text-right indicator-item btn-primary">+</button
                    >
                    <textarea
                        class="textarea w-full"
                        placeholder="konsep"
                        bind:value={keterangan_item}
                    />
                </form>
                <FormAppendCakupan bind:cakupans={cakupan_list} />
                <FormAppendKataAsing bind:kata_asings={kata_asing_list} />
            </div>
        </div>
        <DisplayKonseps konseps={data.konseps} />
    </div>
</div>
