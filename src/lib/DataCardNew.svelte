<script lang="ts">
    // APIs
    import { invoke } from "@tauri-apps/api";
    import _ from "lodash";
    // Components
    import DisplayKonseps from "./components/DisplayKonseps.svelte";
    import FormAppendKonsep from "./components/FormAppendKonsep.svelte";
    import FormAddLemma from "./components/FormAddLemma.svelte";
    // Stores
    // Types
    import type { LemmaItem } from "../bindings/LemmaItem";
    import type { KonsepItem } from "../bindings/KonsepItem";

    // Initialize values
    $: data = {
        id: null,
        lemma: "",
        konseps: [],
    } as LemmaItem;
    let new_konsep = {
        id: null,
        keterangan: "",
        golongan_kata: "",
        cakupans: [],
        kata_asing: [],
    } as KonsepItem;
    // Event listeners
    // Callables
    async function insert_lemma() {
        await invoke("insert_lemma", { item: data });
        data = {
            id: null,
            lemma: "",
            konseps: [],
        } as LemmaItem;
    }
    function append_new_konsep() {
        data.konseps.push(new_konsep);
        data = data;
        new_konsep = {} as KonsepItem;
    }
</script>

<div class="relative w-[35em]">
    <div class="card card-normal m-4 bg-blue-100 shadow-xl">
        <div class="card-body">
            <FormAddLemma bind:lemma={data.lemma} submit={insert_lemma} />
            <DisplayKonseps konseps={data.konseps} editable={true} />
            <FormAppendKonsep
                bind:konsep={new_konsep}
                bind:index={data.konseps.length}
                onSubmit={append_new_konsep}
            />
        </div>
    </div>
</div>
