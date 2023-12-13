<script lang="ts">
    import _ from "lodash";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    import type { LemmaItem } from "../bindings/LemmaItem";
    import type { KonsepItem } from "../bindings/KonsepItem";
    import SelectGolonganKata from "../components/SelectGolonganKata.svelte";
    import FormAppendCakupan from "../components/FormAppendCakupan.svelte";
    import FormAppendKataAsing from "../components/FormAppendKataAsing.svelte";

    export let data: LemmaItem;
    let old_data: LemmaItem;
    let new_keterangan: string;
    let new_golongan_kata: string;
    $: new_cakupans = [];
    $: new_kata_asings = [];

    onMount(() => {
        old_data = _.cloneDeep(data);
    });
    function append_new_konsep() {
        data.konseps.push(
            Object.assign({} as KonsepItem, {
                id: null,
                keterangan: new_keterangan,
                golongan_kata: new_golongan_kata,
                cakupans: new_cakupans,
                kata_asing: new_kata_asings,
            }),
        );
        data = data; // To force reload
        console.log(data);
        new_cakupans = [];
        new_kata_asings = [];
        new_keterangan = "";
    }

    async function submit_changes() {
        console.log(old_data);
        let a = await invoke("submit_changes", { old: old_data, new: data });
        console.log(a);
        console.log("Here!");
    }

    export let toggle_display: CallableFunction;
</script>

<div class="card card-normal m-4 w-[40em] bg-blue-100 shadow-xl">
    <div class="card-body">
        <div class="card-body">
            <div class="join">
                <input
                    type="text"
                    bind:value={data.lemma}
                    class="input input-bordered w-full join-item"
                />
                <button
                    class="btn-primary join-item"
                    on:click={submit_changes}
                    on:click={toggle_display}
                >
                    Save
                </button>
            </div>
        </div>
        <div class="w-full">
            {#if data.konseps != null}
                <div class="text-left p-2 rounded-lg w-full">
                    {#each data.konseps as konsep, i}
                        {i + 1}.
                        <span class="badge">{konsep.golongan_kata}</span>
                        <div>
                            <textarea
                                class="textarea w-full"
                                bind:value={konsep.keterangan}
                            />
                            {#if konsep.cakupans != null}
                                <div class="column">
                                    {#each konsep.cakupans as cakupan}
                                        <div class="badge">{cakupan}</div>
                                    {/each}
                                </div>
                            {:else}{/if}
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
                </div>
            {/if}
        </div>
        <div class="indicator w-full mt-4">
            <div
                class="text-left card-bordered p-2 border-primary border-2 rounded-lg w-full"
            >
                <form
                    on:submit|preventDefault={append_new_konsep}
                    class="space-y-4"
                >
                    [+] {data.konseps.length + 1}.
                    <SelectGolonganKata
                        bind:golongan_kata={new_golongan_kata}
                    />
                    <button
                        type="submit"
                        class="text-right indicator-item btn-primary">+</button
                    >
                    <textarea
                        class="textarea w-full"
                        placeholder="konsep"
                        bind:value={new_keterangan}
                    />
                </form>
                <FormAppendCakupan bind:cakupans={new_cakupans} />
                <FormAppendKataAsing bind:kata_asings={new_kata_asings} />
            </div>
        </div>
    </div>
</div>
