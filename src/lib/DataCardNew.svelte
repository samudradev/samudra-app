<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import _ from "lodash";
    import type { KonsepItem } from "../bindings/KonsepItem";
    import { invoke } from "@tauri-apps/api";
    import type { KataAsingItem } from "../bindings/KataAsingItem";

    let data: LemmaItem = {};
    $: konseps = [];
    let new_keterangan: string;
    let new_golongan_kata: string;
    $: new_cakupans = [];
    let new_cakupan: string;
    $: new_kata_asings = [];
    let new_kata_asing: KataAsingItem = { nama: "", bahasa: "" };

    function append_new_konsep() {
        konseps.push(
            Object.assign({} as KonsepItem, {
                id: null,
                keterangan: new_keterangan,
                golongan_kata: new_golongan_kata,
                cakupans: new_cakupans,
                kata_asing: new_kata_asings,
            }),
        );
        konseps = konseps; // To force reload
        new_cakupans = [];
        new_kata_asings = [];
        new_keterangan = "";
    }
    function append_new_cakupan() {
        new_cakupans.push(new_cakupan);
        new_cakupans = new_cakupans; // To force reload
        new_cakupan = "";
    }
    function append_new_kata_asing() {
        new_kata_asings.push(new_kata_asing);
        new_kata_asings = new_kata_asings; // To force reload
        new_kata_asing = { nama: "", bahasa: "" };
    }

    function append_to_field(item: KonsepItem, field: string, value: string) {
        item[field].push(value);
        konseps = konseps;
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
            <form on:submit|preventDefault={append_new_konsep}>
                [+] {" "}
                <select class="select" bind:value={new_golongan_kata}>
                    <option value="NAMA">NAMA</option>
                </select>
                <button type="submit">+</button>
                <textarea class="textarea" bind:value={new_keterangan} />
            </form>
            <form on:submit|preventDefault={append_new_cakupan}>
                <div class="label">
                    <span class="label-text-alt">Cakupan</span>
                </div>
                <span class="join">
                    <input
                        type="text"
                        placeholder="cakupan"
                        class="textarea join-item"
                        bind:value={new_cakupan}
                    />
                    <button type="submit" class="join-item">+</button>
                </span>
                {#each new_cakupans as cakupan}
                    <div>{cakupan}</div>
                {/each}
            </form>
            <form
                on:submit|preventDefault={append_new_kata_asing}
                class="form-control"
            >
                <div class="label">
                    <span class="label-text-alt">Kata asing</span>
                </div>
                <span class="join">
                    <input
                        type="text"
                        placeholder="kata"
                        class="textarea join-item"
                        bind:value={new_kata_asing.nama}
                    />
                    <input
                        type="text"
                        placeholder="bahasa"
                        class="textarea join-item"
                        bind:value={new_kata_asing.bahasa}
                    />
                    <button type="submit" class="join-item">+</button>
                </span>
                {#each new_kata_asings as kata_asing}
                    <div>{kata_asing.nama}: {kata_asing.bahasa}</div>
                {/each}
            </form>
        </div>
        {#each konseps as konsep, i}
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
            </div>
        {/each}
    </div>
</div>
