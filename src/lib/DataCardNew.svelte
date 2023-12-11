<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import _ from "lodash";
    import type { KonsepItem } from "../bindings/KonsepItem";
    import { invoke } from "@tauri-apps/api";

    let data: LemmaItem = {};
    let konseps: KonsepItem[] = [];
    let new_keterangan: string;
    let new_golongan_kata: string;

    function append_new_konsep() {
        konseps.push(
            Object.assign({} as KonsepItem, {
                id: null,
                keterangan: new_keterangan,
                golongan_kata: new_golongan_kata,
                cakupans: [],
                kata_asing: [],
            }),
        );
        new_keterangan = "";
    }

    async function insert_lemma() {
        data.konseps = konseps;

        data.id = null;
        console.table(data);
        let a = await invoke("insert_lemma", { item: data });
        console.log(a);
        console.log("Here!");
    }
</script>

<div class="card card-normal m-4 w-96 bg-blue-100 shadow-xl">
    <div class="card-body">
        <div class="grid w-full grid-flow-row-dense grid-cols-4 grid-rows-1">
            <input
                type="text"
                bind:value={data.lemma}
                class="input input-bordered w-full max-w-xs card-title col-span-3"
            />
            <button
                class="btn btn-ghost btn-xs col-span-1"
                on:click={insert_lemma}
            >
                Save
            </button>
        </div>
        <div class="text-left">
            [+] {" "}
            <select class="select" bind:value={new_golongan_kata}>
                <option value="NAMA">NAMA</option>
            </select>
            <form on:submit|preventDefault={append_new_konsep}>
                <textarea class="textarea" bind:value={new_keterangan} />
                <button type="submit">+</button>
            </form>
        </div>
        <!-- TODO reactively appear after previous insertion -->
        <!-- {#each konseps as konsep, i}
            <div class="text-left">
                {i + 1}.
                <span class="badge">{konsep.golongan_kata}</span>
                <div>
                    <textarea class="textarea" bind:value={konsep.keterangan} />
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
            </div>
        {/each} -->
    </div>
</div>
