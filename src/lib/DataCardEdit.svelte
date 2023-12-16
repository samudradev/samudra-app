<script lang="ts">
    // APIs
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import _ from "lodash";
    // Components
    import FormAppendKonsep from "./components/FormAppendKonsep.svelte";
    import DisplayKonseps from "./components/DisplayKonseps.svelte";
    import FormAddLemma from "./components/FormAddLemma.svelte";
    // Stores
    import LemmaStore from "./stores/LemmaStore";
    // Types
    import type { LemmaItem } from "../bindings/LemmaItem";

    // Initialize values
    export let data: LemmaItem;
    export let toggle_display: VoidFunction;
    let old_data: LemmaItem;

    onMount(() => {
        old_data = _.cloneDeep(data);
    });
    // Event listeners
    // Callable
    async function submit_changes() {
        await invoke("submit_changes", { old: old_data, new: data });
    }

    async function delete_lemma() {
        await invoke("delete_lemma", { item: data });
        LemmaStore.update((value) => {
            return value.filter((item) => {
                if (item.id != data.id) {
                    return item;
                }
            });
        });
    }

    function cancel() {
        data = _.cloneDeep(old_data);
    }
</script>

<div class="relative w-[35em]">
    <div class="card card-normal m-4 bg-blue-100 shadow-xl">
        <div class="card-body">
            <FormAddLemma
                bind:lemma={data.lemma}
                submit={submit_changes}
                toggle={toggle_display}
            />
            <DisplayKonseps bind:konseps={data.konseps} />
            <FormAppendKonsep bind:data />
        </div>
    </div>
    <div class="menu menu-vertical absolute -right-[6em] top-0 py-[1em]">
        <button class="my-[0.2em]" on:click={cancel} on:click={toggle_display}
            >Cancel</button
        >
        <button
            class="my-[0.2em]"
            on:click={delete_lemma}
            on:click={toggle_display}>Delete</button
        >
    </div>
</div>
