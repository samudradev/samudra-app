<script lang="ts">
    import type { LemmaData } from "../bindings/LemmaData";
    import _ from "lodash";
    import type { KonsepData } from "../bindings/KonsepData";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    export let data: LemmaData;
    let old_data: LemmaData;
    let new_keterangan: string;
    let new_golongan_kata: string;

    onMount(() => {
        old_data = _.cloneDeep(data);
    });
    function append_new_konsep() {
        data.konseps.push(
            Object.assign({} as KonsepData, {
                id: 0,
                keterangan: new_keterangan,
                golongan_kata: {
                    id: new_golongan_kata,
                    keterangan: "",
                },
            })
        );
        data = data;
        new_keterangan = "";
    }

    async function submit_changes() {
        console.log(old_data);
        let a = await invoke("submit_changes", { old: old_data, new: data });
        console.log(a);
        console.log("Here!");
    }

    export let toggle_display: CallableFunction;
    //     TODO Edit logic and fields
    //     TODO Append new values
</script>

<div class="card card-normal m-4 w-96 bg-blue-100 shadow-xl">
    <div class="card-body">
        <div class="grid w-full grid-flow-row-dense grid-cols-4 grid-rows-1">
            <!--            <h2 class="">{data.nama}</h2>-->
            <input
                type="text"
                bind:value={data.nama}
                class="input input-bordered w-full max-w-xs card-title col-span-3"
            />
            <button
                class="btn btn-ghost btn-xs col-span-1"
                on:click={submit_changes}
                on:click={toggle_display}
            >
                Save
            </button>
        </div>
        {#if data.konseps != null}
            {#each data.konseps as konsep, i}
                <div class="text-left">
                    {i + 1}.
                    <span class="badge">{konsep.golongan_kata.id}</span>
                    <div>
                        <textarea
                            class="textarea"
                            bind:value={konsep.keterangan}
                        />
                        {#if konsep.cakupan != null}
                            <div class="column">
                                {#each konsep.cakupan as cakupan}
                                    <div class="badge">{cakupan.nama}</div>
                                {/each}
                            </div>
                        {:else}{/if}
                        {#if konsep.kata_asing != null}
                            <div class="column">
                                {#each konsep.kata_asing as kata_asing}
                                    <div class="badge">{kata_asing.nama}</div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                </div>
            {/each}
        {/if}
        <div class="text-left">
            [+] {data.konseps.length + 1}.
            <select class="select" bind:value={new_golongan_kata}>
                <option value="NAMA">NAMA</option>
            </select>
            <form on:submit|preventDefault={append_new_konsep}>
                <textarea class="textarea" bind:value={new_keterangan} />
                <button type="submit">+</button>
            </form>
        </div>
    </div>
</div>
