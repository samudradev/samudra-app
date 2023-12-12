<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import _ from "lodash";
    import type { KonsepItem } from "../bindings/KonsepItem";
    import { invoke } from "@tauri-apps/api";
    import type { KataAsingItem } from "../bindings/KataAsingItem";

    $: data = {
        id: null,
        lemma: "",
        konseps: [],
    } as LemmaItem;
    let keterangan_item: string;
    let golongan_kata_item: string;
    $: new_cakupans = [];
    let cakupan_item: string;
    $: new_kata_asings = [];
    let kata_asing_item: KataAsingItem = { nama: "", bahasa: "" };

    function append_new_konsep() {
        data.konseps.push(
            Object.assign({} as KonsepItem, {
                id: null,
                keterangan: keterangan_item,
                golongan_kata: golongan_kata_item,
                cakupans: new_cakupans,
                kata_asing: new_kata_asings,
            }),
        );
        data = data; // To force reload
        new_cakupans = [];
        new_kata_asings = [];
        keterangan_item = "";
    }
    function append_new_cakupan() {
        new_cakupans.push(cakupan_item);
        new_cakupans = new_cakupans; // To force reload
        cakupan_item = "";
    }
    function append_new_kata_asing() {
        new_kata_asings.push(kata_asing_item);
        new_kata_asings = new_kata_asings; // To force reload
        kata_asing_item = { nama: "", bahasa: "" };
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
                    <select class="select" bind:value={golongan_kata_item}>
                        <option value="NAMA">NAMA</option>
                    </select>
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
                <form
                    on:submit|preventDefault={append_new_cakupan}
                    class="w-full"
                >
                    <div class="label">
                        <span class="label-text-alt">Cakupan</span>
                    </div>
                    <span class="join w-full">
                        <input
                            type="text"
                            placeholder="cakupan"
                            class="textarea join-item w-full"
                            bind:value={cakupan_item}
                        />
                        <button type="submit" class="join-item">+</button>
                    </span>
                    {#each new_cakupans as cakupan}
                        <div>{cakupan}</div>
                    {/each}
                </form>
                <form
                    on:submit|preventDefault={append_new_kata_asing}
                    class="form-control w-full"
                >
                    <div class="label">
                        <span class="label-text-alt">Kata asing</span>
                    </div>
                    <span class="join w-full">
                        <input
                            type="text"
                            placeholder="kata"
                            class="textarea join-item w-1/2"
                            bind:value={kata_asing_item.nama}
                        />
                        <input
                            type="text"
                            placeholder="bahasa"
                            class="textarea join-item w-1/2"
                            bind:value={kata_asing_item.bahasa}
                        />
                        <button type="submit" class="join-item">+</button>
                    </span>
                    {#each new_kata_asings as kata_asing}
                        <div>{kata_asing.nama}: {kata_asing.bahasa}</div>
                    {/each}
                </form>
            </div>
        </div>
        {#each data.konseps as konsep, i}
            <div class="text-left">
                {i + 1}.
                <span class="badge">{konsep.golongan_kata}</span>
                <div>
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
            </div>
        {/each}
    </div>
</div>
