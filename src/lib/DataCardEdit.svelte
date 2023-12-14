<script lang="ts">
    import _ from "lodash";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    import type { LemmaItem } from "../bindings/LemmaItem";
    import FormAppendKonsep from "../components/FormAppendKonsep.svelte";
    import DisplayKonseps from "../components/DisplayKonseps.svelte";
    import FormAddLemma from "../components/FormAddLemma.svelte";
    import LemmaStore from "../Data";

    export let data: LemmaItem;
    let old_data: LemmaItem;

    onMount(() => {
        old_data = _.cloneDeep(data);
    });

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

    export let toggle_display;
</script>

<div class="card card-normal m-4 w-[40em] bg-blue-100 shadow-xl">
    <div class="card-body">
        <FormAddLemma
            bind:lemma={data.lemma}
            submit={submit_changes}
            toggle={toggle_display}
        />
        <button on:click={delete_lemma} on:click={toggle_display}>Delete</button
        >
        <DisplayKonseps bind:konseps={data.konseps} />
        <FormAppendKonsep bind:data />
    </div>
</div>
