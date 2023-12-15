<script lang="ts">
    import type { LemmaItem } from "../bindings/LemmaItem";
    import _ from "lodash";
    import { invoke } from "@tauri-apps/api";
    import DisplayKonseps from "../components/DisplayKonseps.svelte";
    import FormAppendKonsep from "../components/FormAppendKonsep.svelte";
    import FormAddLemma from "../components/FormAddLemma.svelte";

    $: data = {
        id: null,
        lemma: "",
        konseps: [],
    } as LemmaItem;

    async function insert_lemma() {
        await invoke("insert_lemma", { item: data });
        data = {
            id: null,
            lemma: "",
            konseps: [],
        } as LemmaItem;
    }
</script>

<div class="relative w-[35em]">
    <div class="card card-normal m-4 bg-blue-100 shadow-xl">
        <div class="card-body">
            <FormAddLemma bind:lemma={data.lemma} submit={insert_lemma} />
            <DisplayKonseps konseps={data.konseps} />
            <FormAppendKonsep bind:data />
        </div>
    </div>
</div>
